use std::sync::Arc;

use my_tcp_sockets::tcp_connection::SocketConnection;
use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TunnelConnectionForCorePurposes {
    tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
}

impl TunnelConnectionForCorePurposes {
    pub fn new(
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self { tunnel_connection }
    }

    pub async fn send_connect_to(&self, id: u32, remote_host_port: &str) {
        self.tunnel_connection
            .send(TunnelTcpContract::ConnectTo {
                id,
                remote_host_port: remote_host_port.to_string(),
            })
            .await;
    }

    pub async fn send_disconnected_from_side_a(&self, id: u32) {
        self.tunnel_connection
            .send(TunnelTcpContract::DisconnectedFromSideA(id))
            .await;
    }
}
