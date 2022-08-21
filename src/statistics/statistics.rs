use crate::settings_model::SettingsModel;

use super::{TargetConnections, TrafficHistory};

pub struct Statistics {
    pub traffic_history: TrafficHistory,
    pub target_connections: TargetConnections,
}

impl Statistics {
    pub fn new(settings_model: &SettingsModel) -> Self {
        Self {
            traffic_history: TrafficHistory::new(),
            target_connections: TargetConnections::new(settings_model),
        }
    }

    pub async fn one_second_tick(&self) {
        self.traffic_history.one_minute_tick().await;
    }
}
