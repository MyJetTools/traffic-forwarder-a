use std::sync::Arc;

use my_tcp_sockets::{tcp_connection::SocketConnection, ConnectionEvent, SocketEventCallback};

use crate::app::AppContext;

use traffic_forwarder_shared::tcp_tunnel::{TunnelTcpContract, TunnelTcpSerializer};

pub struct TunnelTcpEvents {
    app: Arc<AppContext>,
}

impl TunnelTcpEvents {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }

    async fn handle_payload(
        &self,
        connection: Arc<SocketConnection<TunnelTcpContract, TunnelTcpSerializer>>,
        payload: TunnelTcpContract,
    ) {
        match payload {
            TunnelTcpContract::Ping => {
                connection.send(TunnelTcpContract::Pong).await;
            }
            TunnelTcpContract::Pong => {
                //B to A packet. Do nothing.
            }
            TunnelTcpContract::ConnectTo { id: _, url: _ } => {
                //B to A packet. Do nothing.
            }
            TunnelTcpContract::Connected(id) => {
                // Confirmation that connection is established on b side.

                println!("Connection {} is established on side b", id);
            }
            TunnelTcpContract::CanNotConnect { id, reason } => {
                // Confirmation that connection can not be established on b side.

                println!(
                    "Connection {} can not be established on side b. Reason: {}",
                    id, reason
                );

                self.app
                    .tunnel_tcp_connection
                    .disconnect_tcp_connection(id)
                    .await;
            }
            TunnelTcpContract::Disconnected(id) => {
                // Socket is disconnected on b side

                println!("Connection {} is disconnected", id);

                self.app
                    .tunnel_tcp_connection
                    .disconnect_tcp_connection(id)
                    .await;
            }
            TunnelTcpContract::Payload { id, payload } => {
                // We have payload from b to a;

                self.app
                    .tunnel_tcp_connection
                    .send_payload_to_target(id, payload)
                    .await;
            }
            TunnelTcpContract::Greeting(handshake_phrase) => {
                if self.app.settings.tunnel_hand_shake_phrase == handshake_phrase {
                    self.app
                        .tunnel_tcp_connection
                        .new_connection_is_established(connection)
                        .await
                } else {
                    println!("Handshake phrase is not correct. Connection is closed.");
                    connection.disconnect().await;
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl SocketEventCallback<TunnelTcpContract, TunnelTcpSerializer> for TunnelTcpEvents {
    async fn handle(
        &self,
        connection_event: ConnectionEvent<TunnelTcpContract, TunnelTcpSerializer>,
    ) {
        match connection_event {
            ConnectionEvent::Connected(_connection) => {}
            ConnectionEvent::Disconnected(_connection) => {
                self.app
                    .tunnel_tcp_connection
                    .tunnel_is_disconnected()
                    .await;
            }
            ConnectionEvent::Payload {
                connection,
                payload,
            } => {
                self.handle_payload(connection, payload).await;
            }
        }
    }
}
