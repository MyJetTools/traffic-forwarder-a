use super::TrafficHistory;

pub struct Statistics {
    pub traffic_history: TrafficHistory,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            traffic_history: TrafficHistory::new(),
        }
    }

    pub async fn one_second_tick(&self) {
        self.traffic_history.one_minute_tick().await;
    }
}
