mod core;
mod services;

pub use self::{
    core::{ConfigBuilderExt, Info as ResumeInfo},
    services::{ShardHandle, ShardRestartKind},
};

use anyhow::Context as _;
use std::{convert::identity, env, pin::pin, time::Duration};
use tokio::{runtime::Builder as RuntimeBuilder, signal};
use tokio_stream::StreamExt as _;
use tracing::{Instrument as _, instrument::Instrumented};
use twilight_gateway::{ConfigBuilder, Event, Shard, queue::InMemoryQueue};
use twilight_http::Client;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN").context("reading `TOKEN`")?;

    let rt = RuntimeBuilder::new_current_thread().enable_all().build()?;
    let _guard = rt.enter();

    let http = Client::new(token.clone());
    let app = rt
        .block_on(async { anyhow::Ok(http.current_user_application().await?.model().await?) })
        .context("getting app")?;
    let info = rt
        .block_on(async { anyhow::Ok(http.gateway().authed().await?.model().await?) })
        .context("getting info")?;

    // The queue defaults are static and may be incorrect for large or newly
    // restarted bots.
    let queue = InMemoryQueue::new(
        info.session_start_limit.max_concurrency,
        info.session_start_limit.remaining,
        Duration::from_millis(info.session_start_limit.reset_after),
        info.session_start_limit.total,
    );
    let config = ConfigBuilder::new(token, core::CONFIG_INTENTS)
        .queue(queue)
        .build();
    let shards = core::restore(config, info.shards);

    core::init(app.id, http, shards.len() as u32);

    let resume_info = rt.block_on(event_loop(shards))?;

    core::save(&resume_info).context("saving resume info")?;

    Ok(())
}

async fn event_loop(shards: impl Iterator<Item = Shard>) -> anyhow::Result<Box<[ResumeInfo]>> {
    services::interaction_handler::register()
        .await
        .context("registering commands")?;

    let tasks = shards
        .map(|shard| tokio::spawn(services::run(event_handler, shard, |_shard| ())))
        .collect::<Box<_>>();

    signal::ctrl_c().await?;
    tracing::info!("shutting down; press CTRL-C to abort");

    let results = tokio_stream::iter(tasks).then(identity);
    tokio::select! {
        _ = signal::ctrl_c() => Ok(Box::default()),
        resume_info = results.collect::<Result<_, _>>() => anyhow::Ok(resume_info?),
    }
}

async fn event_handler(event: Event, _state: ()) {
    async fn log_err(future: Instrumented<impl Future<Output = anyhow::Result<()>>>) {
        let mut future = pin!(future);
        if let Err(error) = future.as_mut().await {
            let _enter = future.span().enter();
            tracing::warn!(error = &*error, "failed to handle event");
        }
    }

    match event {
        Event::Ready(rdy) => {
            let span = tracing::info_span!("ready", id = %&rdy.session_id);
            log_err(services::ready_handler::ready_handler(rdy).instrument(span)).await;
        }
        Event::MessageCreate(msg) => {
            let span = tracing::info_span!("message", id = %&msg.id);
            log_err(services::message_handler::message_handler(msg).instrument(span)).await;
        }
        Event::InteractionCreate(interaction) => {
            let span = tracing::info_span!("interaction", id = %interaction.id);
            log_err(services::interaction_handler::handler(interaction).instrument(span)).await;
        }
        _ => {}
    }
}
