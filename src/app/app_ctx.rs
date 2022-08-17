use std::sync::Arc;

use my_tcp_sockets::TcpServer;
use rust_extensions::AppStates;

use crate::{
    settings_model::SettingsModel,
    tcp_tunnel::{TcpTunnelConnection, TunnelTcpContract, TunnelTcpSerializer},
};

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub tunnel_tcp_server: TcpServer<TunnelTcpContract, TunnelTcpSerializer>,
    pub tunnel_tcp_connection: TcpTunnelConnection,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self {
            tunnel_tcp_server: TcpServer::new("TcpTunnel".to_string(), settings.get_tunnel_addr()),
            app_states: Arc::new(AppStates::create_initialized()),
            settings,
            tunnel_tcp_connection: TcpTunnelConnection::new(),
        }
    }
}
