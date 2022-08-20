use my_http_server_swagger::*;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TunnelTrafficHistoryItem {
    #[serde(rename = "i")]
    incoming: usize,
    #[serde(rename = "o")]
    outcoming: usize,
}

impl TunnelTrafficHistoryItem {
    pub async fn crate_vec(app: &AppContext) -> Vec<Self> {
        let mut result = Vec::new();

        for item in app.statistics.traffic_history.get_traffic_history().await {
            result.push(Self {
                incoming: item.incoming,
                outcoming: item.outcoming,
            });
        }

        result
    }
}
