use std::sync::Arc;

use rust_extensions::MyTimerTick;

use crate::app::AppContext;

pub struct OneSecondTimer {
    app: Arc<AppContext>,
}

impl OneSecondTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for OneSecondTimer {
    async fn tick(&self) {
        self.app.statistics.one_second_tick().await;
    }
}
