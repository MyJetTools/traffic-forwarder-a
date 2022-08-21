use my_http_server_swagger::*;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

use super::{ServiceContract, TunnelTrafficHistoryItem};

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct StatusContract {
    #[serde(rename = "tunnelConnected")]
    pub tunnel_connected: bool,
    #[serde(rename = "tunnelTrafficHistory")]
    pub tunnel_traffic_history: Vec<TunnelTrafficHistoryItem>,

    pub services: Vec<ServiceContract>,
}

impl StatusContract {
    pub async fn create(app: &AppContext) -> Self {
        let tunnel_connected = app.tunnel_tcp_connection.is_connection_established();
        Self {
            tunnel_connected,
            tunnel_traffic_history: TunnelTrafficHistoryItem::crate_vec(app).await,
            services: ServiceContract::create_vec(app),
        }
    }
}
