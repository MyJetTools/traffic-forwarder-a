use std::sync::Arc;

use my_tcp_sockets::tcp_connection::SocketConnection;

use crate::tcp_listener::TcpListenerConnections;

use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TcpTunnel {
    pub tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    pub connections: TcpListenerConnections,
}

impl TcpTunnel {
    pub fn new(
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self {
            tunnel_connection,
            connections: TcpListenerConnections::new(),
        }
    }

    pub fn dispose(self) {
        let connections = self.connections.remove_all();

        for (_, connection) in connections {
            connection.disconnect();
        }
    }

    pub fn send_disconnect_to_tunnel(&self, connection_id: u32) {
        let tunnel_connection = self.tunnel_connection.clone();
        tokio::spawn(async move {
            tunnel_connection
                .send(TunnelTcpContract::Disconnected(connection_id))
                .await;
        });
    }

    pub fn send_payload(&self, id: u32, payload: Vec<u8>) {
        let tunnel_connection = self.tunnel_connection.clone();
        tokio::spawn(async move {
            tunnel_connection
                .send(TunnelTcpContract::Payload { id, payload })
                .await;
        });
    }
}
