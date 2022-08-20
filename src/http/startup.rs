use std::{net::SocketAddr, sync::Arc};

use crate::app::AppContext;
use my_http_server::MyHttpServer;
use my_http_server::StaticFilesMiddleware;

pub fn start_http_server(app: &Arc<AppContext>, port: u16) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], port)));

    let controllers = crate::http::build_controllers::build_controllers(app);
    http_server.add_middleware(Arc::new(controllers));

    if cfg!(debug_assertions) {
        http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(
            Some(vec![my_http_server::FilesMapping::new(
                "/typescript",
                "./typescript",
            )]),
            None,
        )));
    } else {
        http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(None, None)));
    }

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
