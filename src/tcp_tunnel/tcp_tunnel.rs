use std::sync::Arc;

use my_tcp_sockets::tcp_connection::SocketConnection;

use crate::target_tcp_listener::TargetTcpListenerConnections;

use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TcpTunnel {
    tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    pub connections: TargetTcpListenerConnections,
}

impl TcpTunnel {
    pub fn new(
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self {
            tunnel_connection,
            connections: TargetTcpListenerConnections::new(),
        }
    }

    pub fn dispose(self) {
        let connections = self.connections.remove_all();

        for (_, connection) in connections {
            connection.disconnect();
        }
    }

    pub fn get_tunnel_connection(
        &self,
    ) -> Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>> {
        self.tunnel_connection.clone()
    }
}
