mod server;
use server::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    serve().await;

    Ok(())
}
