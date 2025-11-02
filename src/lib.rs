pub mod chapter1;
pub mod chapter2;
pub mod configures;

use crate::configures::dbconfig::DBConfig;

pub async fn run_app() {
    // main logic here
    let db_config = DBConfig::init();
    let rbatis = db_config.get_connection().await;
}
