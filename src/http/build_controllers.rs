use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    result.register_get_action(Arc::new(
        crate::http::controllers::home_controller::IndexAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::status_controller::GetStatusAction::new(app.clone()),
    ));

    result
}
