#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  back::start_server().await
}
