#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).unwrap();
    architecture::run_app().await;
}
