#[tokio::main]
async fn main() {
    println!(
        "Current dir: {}",
        std::env::current_dir().unwrap().display()
    );
    architecture::run_app().await;
}
