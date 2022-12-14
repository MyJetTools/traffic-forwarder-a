use std::sync::Arc;

use my_tcp_sockets::TcpServer;
use rust_extensions::AppStates;

use crate::{settings_model::SettingsModel, statistics::Statistics, tcp_tunnel::TcpTunnel};

use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub tunnel_tcp_server: TcpServer<TunnelTcpContract, TunnelTcpSerializer>,
    pub tunnel_tcp_connection: TcpTunnel,
    pub process_id: String,
    pub statistics: Arc<Statistics>,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self {
            statistics: Arc::new(Statistics::new(&settings)),
            tunnel_tcp_server: TcpServer::new("TcpTunnel".to_string(), settings.get_tunnel_addr()),
            app_states: Arc::new(AppStates::create_initialized()),
            settings,
            tunnel_tcp_connection: TcpTunnel::new(),
            process_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}
