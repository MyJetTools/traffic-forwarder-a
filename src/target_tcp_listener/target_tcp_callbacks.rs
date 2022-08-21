use std::sync::Arc;

use crate::app::AppContext;

use super::TargetTcpConnection;

pub struct TargetTcpCallbacks {
    app: Arc<AppContext>,
    remote_host: String,
    port: u16,
}

impl TargetTcpCallbacks {
    pub fn new(app: Arc<AppContext>, port: u16, remote_host: String) -> Self {
        Self {
            app,
            remote_host,
            port,
        }
    }
    pub async fn on_new_connection(
        &self,
        target_tcp_connection: &Arc<TargetTcpConnection>,
    ) -> bool {
        self.app
            .statistics
            .target_connections
            .new_connection(self.port);
        self.app
            .tunnel_tcp_connection
            .new_target_tcp_connection(target_tcp_connection, &self.remote_host)
            .await
            .is_some()
    }
    pub async fn on_new_disconnection(&self, tcp_connection: Arc<TargetTcpConnection>) {
        self.app
            .statistics
            .target_connections
            .new_disconnection(self.port);

        self.app
            .tunnel_tcp_connection
            .target_connection_is_disconnected(
                tcp_connection.id,
                crate::tcp_tunnel::DisconnectReason::DisconnectedFromSideA,
            )
            .await;
    }
    pub async fn on_new_payload(
        &self,
        tcp_connection: &Arc<TargetTcpConnection>,
        payload: Vec<u8>,
    ) -> bool {
        self.app
            .tunnel_tcp_connection
            .send_payload_to_tunnel(tcp_connection.id, payload)
            .await
    }
}
