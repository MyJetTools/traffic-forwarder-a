use std::{net::SocketAddr, sync::Arc};

use app::AppContext;
use tcp_listener::ServiceTcpListener;
use tcp_tunnel::*;
use traffic_forwarder_shared::tcp_tunnel::TunnelTcpSerializer;

mod app;

mod settings_model;
mod tcp_listener;
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

    for service_settings in &app.settings.get_serives() {
        service_sockets.push(ServiceTcpListener::new(
            app.clone(),
            SocketAddr::from(([0, 0, 0, 0], service_settings.port)),
            service_settings.remote_host.clone(),
            1024 * 1024 * 5,
        ));
    }
    for service_socket in &service_sockets {
        service_socket.start();
    }

    app.app_states.wait_until_shutdown().await;
}
