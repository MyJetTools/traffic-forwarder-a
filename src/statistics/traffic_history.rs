use tokio::sync::Mutex;

use super::value_accumulator::ValueAccumulator;

const MAX_ELEMENTS_AMOUNT: usize = 120;

#[derive(Debug, Clone)]
pub struct TrafficHistoryItem {
    pub incoming: usize,
    pub outcoming: usize,
}

pub struct TrafficHistory {
    history: Mutex<Vec<TrafficHistoryItem>>,

    pub incoming_accumulator: ValueAccumulator,
    pub outcoming_accumulator: ValueAccumulator,
}

impl TrafficHistory {
    pub fn new() -> Self {
        Self {
            history: Mutex::new(Vec::with_capacity(MAX_ELEMENTS_AMOUNT)),
            incoming_accumulator: ValueAccumulator::new(),
            outcoming_accumulator: ValueAccumulator::new(),
        }
    }

    pub async fn one_minute_tick(&self) {
        let incoming = self.incoming_accumulator.get_one_second();
        let outcoming = self.outcoming_accumulator.get_one_second();

        let mut history_write_access = self.history.lock().await;
        if history_write_access.len() >= MAX_ELEMENTS_AMOUNT {
            history_write_access.remove(0);
        }
        history_write_access.push(TrafficHistoryItem {
            incoming,
            outcoming,
        });
    }

    pub async fn get_traffic_history(&self) -> Vec<TrafficHistoryItem> {
        let history_write_access = self.history.lock().await;
        let result: &Vec<TrafficHistoryItem> = history_write_access.as_ref();
        result.clone()
    }
}
