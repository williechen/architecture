#[tokio::main]
async fn main() {
    println!("cwd = {:?}", std::env::current_dir().unwrap());
    architecture::run_app().await;
}
