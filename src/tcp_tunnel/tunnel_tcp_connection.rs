use std::sync::{atomic::AtomicBool, Arc};

use my_tcp_sockets::tcp_connection::SocketConnection;
use tokio::sync::Mutex;

use crate::tcp_listener::TcpConnection;

use super::TcpTunnel;
use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TcpTunnelConnection {
    connection_is_established: AtomicBool,
    connection: Mutex<Option<TcpTunnel>>,
}

impl TcpTunnelConnection {
    pub fn new() -> Self {
        Self {
            connection_is_established: AtomicBool::new(false),
            connection: Mutex::new(None),
        }
    }

    pub fn is_connection_established(&self) -> bool {
        self.connection_is_established
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn new_connection_is_established(
        &self,
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) {
        let mut write_access = self.connection.lock().await;

        if let Some(old_tunnel) = write_access.replace(TcpTunnel::new(tunnel_connection)) {
            old_tunnel.dispose();
        }

        self.connection_is_established
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn tunnel_is_disconnected(&self) {
        let mut write_access = self.connection.lock().await;

        if let Some(old_tunnel) = write_access.take() {
            old_tunnel.dispose();
        }

        self.connection_is_established
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn send_payload_to_tunnel(&self, id: u32, payload: &[u8]) -> bool {
        let connection_access = self.connection.lock().await;

        match &*connection_access {
            Some(tunnel) => {
                tunnel.send_payload(id, payload.to_vec());
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub async fn new_tcp_connection(&self, tcp_connection: &Arc<TcpConnection>) -> Option<i32> {
        let mut connection_access = self.connection.lock().await;

        match &mut *connection_access {
            Some(tunnel_connection) => {
                tunnel_connection.connections.add(tcp_connection.clone());
                return Some(tunnel_connection.tunnel_connection.id);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn disconnect_tcp_connection(&self, connection_id: u32) {
        let mut connection_access = self.connection.lock().await;

        if let Some(tunnel) = &mut *connection_access {
            if let Some(removed_connection) = tunnel.connections.remove(connection_id) {
                removed_connection.disconnect();
                tunnel.send_disconnect_to_tunnel(connection_id);
            }
        }
    }

    async fn get_connection(&self, connection_id: u32) -> Option<Arc<TcpConnection>> {
        let connection_access = self.connection.lock().await;

        match &*connection_access {
            Some(tunnel_connection) => {
                return tunnel_connection.connections.get(connection_id);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn send_payload_to_target(&self, connection_id: u32, payload: Vec<u8>) {
        if let Some(connection) = self.get_connection(connection_id).await {
            connection.send_payload(payload)
        }
    }
}
