use std::sync::Arc;

use my_tcp_sockets::tcp_connection::SocketConnection;

use crate::tcp_listener::TcpListenerConnections;

use super::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TcpTunnelConnectionSingleThreaded {
    pub tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    pub connections: TcpListenerConnections,
}

impl TcpTunnelConnectionSingleThreaded {
    pub fn new(
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self {
            tunnel_connection,
            connections: TcpListenerConnections::new(),
        }
    }

    pub async fn send_disconnect_to_tunnel(&self, id: u32) {
        self.tunnel_connection
            .send(TunnelTcpContract::Disconnected(id))
            .await;
    }

    pub async fn send_payload(&self, tunnel_connection_id: i32, id: u32, payload: Vec<u8>) {
        if self.tunnel_connection.id == tunnel_connection_id {
            self.tunnel_connection
                .send(TunnelTcpContract::Payload { id, payload })
                .await;
        }
    }
}
