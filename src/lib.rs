pub mod chapter1;
pub mod chapter2;
pub mod configures;

use crate::configures::application;

pub async fn run_app() {
    let _a = application::get_config().logger.load();
    application::get_config().database.get_connection();

    tracing::info!(
        "Starting server at {} in {} mode",
        application::get_config().server.address(),
        application::get_config().server.app_env()
    );
}
