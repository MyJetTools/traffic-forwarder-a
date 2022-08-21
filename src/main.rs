use std::{net::SocketAddr, sync::Arc, time::Duration};

use app::AppContext;
use rust_extensions::MyTimer;
use target_tcp_listener::{TargetTcpCallbacks, TargetTcpListener};
use tcp_tunnel::*;
use traffic_forwarder_shared::tcp_tunnel::TunnelTcpSerializer;

mod app;
mod background;
mod http;
mod settings_model;
mod statistics;
mod target_tcp_listener;
mod tcp_tunnel;

#[tokio::main]
async fn main() {
    let settings = crate::settings_model::SettingsModel::load(".traffic-forwarder-a").await;

    let app = AppContext::new(settings);

    let app = Arc::new(app);

    let mut service_sockets = Vec::new();

    app.tunnel_tcp_server
        .start(
            Arc::new(TunnelTcpSerializer::new),
            Arc::new(TunnelTcpEvents::new(app.clone())),
            app.app_states.clone(),
            my_logger::LOGGER.clone(),
        )
        .await;

    for service_settings in app.settings.get_serives() {
        service_sockets.push(TargetTcpListener::new(
            app.clone(),
            SocketAddr::from(([0, 0, 0, 0], service_settings.port)),
            service_settings.port,
            service_settings.remote_host,
            1024 * 1024 * 5,
        ));
    }
    for service_socket in &service_sockets {
        service_socket.start(Arc::new(TargetTcpCallbacks::new(
            app.clone(),
            service_socket.listening_port,
            service_socket.remote_host.clone(),
        )));
    }

    let mut timer_1s = MyTimer::new(Duration::from_secs(1));

    timer_1s.register_timer(
        "OneSecTimer",
        Arc::new(crate::background::OneSecondTimer::new(app.clone())),
    );

    timer_1s.start(app.app_states.clone(), my_logger::LOGGER.clone());

    crate::http::start_http_server(&app, app.settings.http_port);

    app.app_states.wait_until_shutdown().await;
}
