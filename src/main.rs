pub(crate) mod api;
pub(crate) mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = api::ApiClient::new("https://api.spacetraders.io");

    tracing::debug!("Factions :");
    for faction in client.get_factions().await?.data() {
        tracing::debug!("{}", faction.name());
    }

    Ok(())
}
