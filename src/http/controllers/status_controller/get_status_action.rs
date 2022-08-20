use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use my_http_server_controllers::controllers::{
    actions::GetAction, documentation::HttpActionDescription,
};

use crate::app::AppContext;

use super::contracts::*;

pub struct GetStatusAction {
    pub app: Arc<AppContext>,
}

impl GetStatusAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl GetAction for GetStatusAction {
    fn get_route(&self) -> &str {
        "/api/status"
    }

    fn get_description(&self) -> Option<HttpActionDescription> {
        None
    }

    async fn handle_request(&self, _: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let result = StatusContract::create(&self.app).await;

        HttpOutput::as_json(result).into_ok_result(false).into()
    }
}
