use twilight_model::gateway::payload::incoming::Ready;

pub async fn ready_handler(event: Ready) -> anyhow::Result<()> {
    tracing::info!(
        "Hello World! I'm ready to party in {} guilds!",
        event.guilds.len()
    );

    Ok(())
}
