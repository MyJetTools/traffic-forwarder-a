use std::sync::Arc;

use my_tcp_sockets::{tcp_connection::SocketConnection, ConnectionEvent, SocketEventCallback};

use crate::{app::AppContext, tcp_tunnel::DisconnectReason};

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
            TunnelTcpContract::ConnectTo {
                id: _,
                remote_host_port: _,
            } => {
                //B to A packet. Do nothing.
            }
            TunnelTcpContract::Connected(id) => {
                // Confirmation that connection is established on b side.

                self.app
                    .tunnel_tcp_connection
                    .confirm_target_connection(id, self.app.statistics.clone())
                    .await;
            }
            TunnelTcpContract::CanNotConnect { id, reason } => {
                // Confirmation that connection can not be established on b side.

                println!(
                    "Connection {} can not be established on side b. Reason: {}",
                    id, reason
                );

                self.app
                    .tunnel_tcp_connection
                    .target_connection_is_disconnected(id, DisconnectReason::CanNotConnect)
                    .await;
            }
            TunnelTcpContract::DisconnectedFromSideA(id) => {
                // Socket is disconnected on b side

                self.app
                    .tunnel_tcp_connection
                    .target_connection_is_disconnected(id, DisconnectReason::DisconnectedFromSideA)
                    .await;
            }

            TunnelTcpContract::DisconnectedFromSideB(id) => {
                // Socket is disconnected on b side

                println!("Connection {} is disconnected size b", id);

                self.app
                    .tunnel_tcp_connection
                    .target_connection_is_disconnected(id, DisconnectReason::DisconnectedFromSideB)
                    .await;
            }
            TunnelTcpContract::Payload { id, payload } => {
                // We have payload from b to a;

                self.app
                    .statistics
                    .traffic_history
                    .incoming_accumulator
                    .append(payload.len());

                self.app
                    .tunnel_tcp_connection
                    .send_payload_to_target(id, payload)
                    .await;
            }
            TunnelTcpContract::Greeting(handshake_phrase) => {
                if self.app.settings.tunnel_hand_shake_phrase == handshake_phrase {
                    self.app
                        .tunnel_tcp_connection
                        .tunnel_connection_is_established(connection)
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
