use std::sync::Arc;

use my_tcp_sockets::tcp_connection::SocketConnection;
use rust_extensions::lazy::LazyVec;
use tokio::sync::Mutex;
use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub enum TunnelConnectionToSendPayload {
    NotInitialized(Mutex<Option<LazyVec<Vec<u8>>>>),
    Initialized(Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>),
}

impl TunnelConnectionToSendPayload {
    pub fn new() -> Self {
        Self::NotInitialized(Mutex::new(Some(LazyVec::new())))
    }
    pub async fn send_payload(&self, connection_id: u32, payload: Vec<u8>) {
        match self {
            TunnelConnectionToSendPayload::NotInitialized(payloads) => {
                payloads.lock().await.as_mut().unwrap().add(payload);
            }
            TunnelConnectionToSendPayload::Initialized(socket) => {
                socket
                    .send(TunnelTcpContract::Payload {
                        id: connection_id,
                        payload,
                    })
                    .await;
            }
        }
    }

    pub async fn get_payloads(&self) -> Option<Vec<Vec<u8>>> {
        match self {
            TunnelConnectionToSendPayload::NotInitialized(payloads) => {
                let mut write_access = payloads.lock().await;
                let result = write_access.take().unwrap();
                result.get_result()
            }
            TunnelConnectionToSendPayload::Initialized(_) => None,
        }
    }

    pub fn create_initialized(
        tunnel_socket: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
    ) -> Self {
        Self::Initialized(tunnel_socket)
    }
}
