use my_http_server_swagger::*;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ServiceContract {
    #[serde(rename = "port")]
    listen_port: u16,
    #[serde(rename = "remoteHost")]
    remote_host: String,
    connections: u16,
}

impl ServiceContract {
    pub fn create_vec(app: &AppContext) -> Vec<Self> {
        let mut result = Vec::new();

        for item in app.statistics.target_connections.get_metrics() {
            result.push(Self {
                listen_port: item.listening_port,
                remote_host: item.remote_host,
                connections: item.connections,
            });
        }

        result
    }
}
