#[tokio::main]
async fn main() -> anyhow::Result<()> {
    flytrap::app::run().await
}
