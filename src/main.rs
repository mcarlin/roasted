use std::error::Error;
use roasted::run_application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_application().await
}
