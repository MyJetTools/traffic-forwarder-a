use std::sync::{atomic::AtomicBool, Arc};

use my_tcp_sockets::tcp_connection::SocketConnection;
use tokio::sync::Mutex;

use crate::{statistics::Statistics, target_tcp_listener::TargetTcpConnection};

use super::{TcpTunnelInner, TunnelConnectionToSendPayload};
use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub enum DisconnectReason {
    CanNotConnect,
    DisconnectedFromSideA,
    DisconnectedFromSideB,
}

pub struct TcpTunnel {
    connection_is_established: AtomicBool,
    tunnel: Mutex<Option<TcpTunnelInner>>,
}

impl TcpTunnel {
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

        if let Some(old_tunnel) = tunnel_access.replace(TcpTunnelInner::new(tunnel_connection)) {
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

    async fn get_tunnel_connection_to_send_payload(
        &self,
        connection_id: u32,
    ) -> Option<Arc<TunnelConnectionToSendPayload>> {
        let mut tunnel_access = self.tunnel.lock().await;

        match tunnel_access.as_mut() {
            Some(tunnel) => {
                return Some(tunnel.get_tunnel_connection_to_send_payload(connection_id));
            }
            None => {
                return None;
            }
        }
    }

    pub async fn send_payload_to_tunnel(&self, id: u32, payload: Vec<u8>) -> bool {
        if let Some(tunnel_connection) = self.get_tunnel_connection_to_send_payload(id).await {
            tunnel_connection.send_payload(id, payload).await;
            return true;
        } else {
            return false;
        }
    }

    pub async fn new_target_tcp_connection(
        &self,
        tcp_connection: &Arc<TargetTcpConnection>,
        remote_host_port: &str,
    ) -> Option<TcpTunnel> {
        let connection = {
            let mut tunnel_access = self.tunnel.lock().await;

            match &mut *tunnel_access {
                Some(tcp_tunnel) => {
                    tcp_tunnel.connections.add(tcp_connection.clone());

                    Some(tcp_tunnel.get_tunnel_connection_for_core_purposes())
                }
                None => None,
            }
        };

        if let Some(connection) = connection {
            connection
                .send_connect_to(tcp_connection.id, remote_host_port)
                .await;

            return Some(TcpTunnel::new());
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
                tunnel.target_connection_is_disconnected(connection_id);
                if let Some(removed_connection) = tunnel.connections.remove(connection_id) {
                    removed_connection.disconnect();
                    Some(tunnel.get_tunnel_connection_for_core_purposes())
                } else {
                    println!("Connection {} not found to disconnect", connection_id);
                    None
                }
            } else {
                None
            }
        };

        if let DisconnectReason::DisconnectedFromSideA = reason {
            if let Some(tunnel_connection) = tunnel_connection {
                tunnel_connection
                    .send_disconnected_from_side_a(connection_id)
                    .await;
            }
        }
    }

    async fn get_target_connection(&self, connection_id: u32) -> Option<Arc<TargetTcpConnection>> {
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
        if let Some(connection) = self.get_target_connection(connection_id).await {
            connection.send_payload(payload)
        }
    }

    pub async fn confirm_target_connection(&self, connection_id: u32, statistics: Arc<Statistics>) {
        let payloads_to_send = {
            let mut tunnel_access = self.tunnel.lock().await;

            if let Some(tunnel_access) = tunnel_access.as_mut() {
                tunnel_access
                    .confirm_target_connection_is_connected(connection_id, statistics)
                    .await
            } else {
                None
            }
        };

        if let Some((connection, payloads_to_send)) = payloads_to_send {
            for payload in payloads_to_send {
                connection.send_payload(connection_id, payload).await;
            }
        }
    }
}
