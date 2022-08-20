use std::{collections::HashMap, sync::Arc};

use my_tcp_sockets::tcp_connection::SocketConnection;

use crate::{statistics::Statistics, target_tcp_listener::TargetTcpListenerConnections};

use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

use super::{TunnelConnectionForCorePurposes, TunnelConnectionToSendPayload};

pub struct TcpTunnelInner {
    tunnel_connection_for_core_purposes: Arc<TunnelConnectionForCorePurposes>,
    tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    tunnel_connections_to_send_payload: HashMap<u32, Arc<TunnelConnectionToSendPayload>>,
    pub connections: TargetTcpListenerConnections,
}

impl TcpTunnelInner {
    pub fn new(
        tunnel_connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self {
            tunnel_connection_for_core_purposes: Arc::new(TunnelConnectionForCorePurposes::new(
                tunnel_connection.clone(),
            )),
            tunnel_connection,
            connections: TargetTcpListenerConnections::new(),
            tunnel_connections_to_send_payload: HashMap::new(),
        }
    }

    pub fn dispose(self) {
        let connections = self.connections.remove_all();

        for (_, connection) in connections {
            connection.disconnect();
        }
    }

    pub fn get_tunnel_connection_to_send_payload(
        &mut self,
        connection_id: u32,
    ) -> Arc<TunnelConnectionToSendPayload> {
        if let Some(connection) = self.tunnel_connections_to_send_payload.get(&connection_id) {
            return connection.clone();
        }

        let not_initialized_connection = Arc::new(TunnelConnectionToSendPayload::new());

        self.tunnel_connections_to_send_payload
            .insert(connection_id, not_initialized_connection.clone());

        not_initialized_connection
    }

    pub fn get_tunnel_connection_for_core_purposes(&self) -> Arc<TunnelConnectionForCorePurposes> {
        self.tunnel_connection_for_core_purposes.clone()
    }

    pub async fn confirm_target_connection_is_connected(
        &mut self,
        connection_id: u32,
        statistics: Arc<Statistics>,
    ) -> Option<(Arc<TunnelConnectionToSendPayload>, Vec<Vec<u8>>)> {
        if let Some(old_connection) = self
            .tunnel_connections_to_send_payload
            .remove(&connection_id)
        {
            let result = old_connection.get_payloads().await;

            let connection = Arc::new(TunnelConnectionToSendPayload::create_initialized(
                self.tunnel_connection.clone(),
                statistics,
            ));

            self.tunnel_connections_to_send_payload
                .insert(connection_id, connection.clone());

            if let Some(result) = result {
                Some((connection, result))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn target_connection_is_disconnected(&mut self, connection_id: u32) {
        self.tunnel_connections_to_send_payload
            .remove(&connection_id);
    }
}
