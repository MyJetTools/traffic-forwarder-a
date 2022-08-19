use std::sync::{atomic::AtomicBool, Arc};

use my_tcp_sockets::tcp_connection::SocketConnection;
use tokio::sync::Mutex;

use crate::target_tcp_listener::TargetTcpConnection;

use super::TcpTunnel;
use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub enum DisconnectReason {
    CanNotConnect,
    DisconnectedFromSideA,
    DisconnectedFromSideB,
}

pub struct TcpTunnelConnection {
    connection_is_established: AtomicBool,
    tunnel: Mutex<Option<TcpTunnel>>,
}

impl TcpTunnelConnection {
    pub fn new() -> Self {
        Self {
            connection_is_established: AtomicBool::new(false),
            tunnel: Mutex::new(None),
        }
    }

    pub fn is_connection_established(&self) -> bool {
        self.connection_is_established
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn tunnel_connection_is_established(
        &self,
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) {
        let mut tunnel_access = self.tunnel.lock().await;

        if let Some(old_tunnel) = tunnel_access.replace(TcpTunnel::new(tunnel_connection)) {
            old_tunnel.dispose();
        }

        self.connection_is_established
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn tunnel_is_disconnected(&self) {
        let mut tunnel_access = self.tunnel.lock().await;

        if let Some(old_tunnel) = tunnel_access.take() {
            old_tunnel.dispose();
        }

        self.connection_is_established
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    async fn get_tunnel_connection(
        &self,
    ) -> Option<Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>> {
        let tunnel_access = self.tunnel.lock().await;

        match &*tunnel_access {
            Some(tunnel) => {
                return Some(tunnel.get_tunnel_connection());
            }
            None => {
                return None;
            }
        }
    }

    pub async fn send_payload_to_tunnel(&self, id: u32, payload: Vec<u8>) -> bool {
        if let Some(tunnel_connection) = self.get_tunnel_connection().await {
            tunnel_connection
                .send(TunnelTcpContract::Payload { id, payload })
                .await;
            return true;
        } else {
            return false;
        }
    }

    pub async fn new_target_tcp_connection(
        &self,
        tcp_connection: &Arc<TargetTcpConnection>,
        url: &str,
    ) -> Option<TcpTunnelConnection> {
        let connection = {
            let mut tunnel_access = self.tunnel.lock().await;

            match &mut *tunnel_access {
                Some(tcp_tunnel) => {
                    tcp_tunnel.connections.add(tcp_connection.clone());

                    Some(tcp_tunnel.get_tunnel_connection())
                }
                None => None,
            }
        };

        if let Some(connection) = connection {
            connection
                .send(TunnelTcpContract::ConnectTo {
                    id: tcp_connection.id,
                    url: url.to_string(),
                })
                .await;

            return Some(TcpTunnelConnection::new());
        } else {
            return None;
        }
    }

    pub async fn target_connection_is_disconnected(
        &self,
        connection_id: u32,
        reason: DisconnectReason,
    ) {
        let tunnel_connection = {
            let mut tunnel_access = self.tunnel.lock().await;

            if let Some(tunnel) = &mut *tunnel_access {
                if let Some(removed_connection) = tunnel.connections.remove(connection_id) {
                    removed_connection.disconnect();
                    Some(tunnel.get_tunnel_connection())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let DisconnectReason::DisconnectedFromSideA = reason {
            if let Some(tunnel_connection) = tunnel_connection {
                tunnel_connection
                    .send(TunnelTcpContract::DisconnectedFromSideA(connection_id))
                    .await;
            }
        }
    }

    async fn get_connection(&self, connection_id: u32) -> Option<Arc<TargetTcpConnection>> {
        let tunnel_access = self.tunnel.lock().await;

        match &*tunnel_access {
            Some(tunnel_access) => {
                return tunnel_access.connections.get(connection_id);
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
