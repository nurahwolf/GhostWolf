mod repeat;
mod repeat_message;
mod restart;
mod send;
pub mod twee;

use crate::{
    core::{CTX, GUILD_COZY, InteractionService},
    services::Ping,
};
use twilight_model::{
    application::interaction::{InteractionData, InteractionType},
    gateway::payload::incoming::InteractionCreate,
};

pub async fn register() -> anyhow::Result<()> {
    let cap = CTX.shards.capacity() as u32;

    CTX.interaction()
        .set_global_commands(&[Ping::command_def()])
        .await?;
    CTX.interaction()
        .set_guild_commands(
            GUILD_COZY,
            &[
                restart::command(cap),
                send::command(),
                repeat::command(),
                repeat_message::command(),
                twee::command(),
            ],
        )
        .await?;

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Ping,
    Restart,
    Send,
    Repeat,
    RepeatMessage,
    Twee,
}

impl From<&str> for Kind {
    fn from(name: &str) -> Self {
        match name {
            Ping::NAME => Kind::Ping,
            restart::NAME => Kind::Restart,
            send::NAME => Kind::Send,
            repeat::NAME => Kind::Repeat,
            repeat_message::NAME => Kind::RepeatMessage,
            twee::NAME => Kind::Twee,
            _ => panic!("unknown command name: '{name}'"),
        }
    }
}

pub async fn handler(mut interaction: Box<InteractionCreate>) -> anyhow::Result<()> {
    match interaction.kind {
        InteractionType::ApplicationCommandAutocomplete => {
            let InteractionData::ApplicationCommand(data) = interaction.data.take().unwrap() else {
                unreachable!();
            };
            let kind = data.name.as_str().into();

            match kind {
                Kind::Ping => unreachable!(),
                Kind::Restart => restart::autocomplete(interaction, data).await?,
                Kind::Send => send::autocomplete(interaction, data).await?,
                Kind::Repeat => repeat::autocomplete(interaction, data).await?,
                Kind::RepeatMessage => repeat_message::autocomplete(interaction, data).await?,
                Kind::Twee => twee::autocomplete(interaction, data).await?,
            }
        }
        InteractionType::ApplicationCommand => {
            let InteractionData::ApplicationCommand(data) = interaction.data.take().unwrap() else {
                unreachable!();
            };
            let kind = data.name.as_str().into();

            match kind {
                Kind::Ping => Ping {}.run(interaction, data).await?,
                Kind::Restart => restart::run(interaction, data).await?,
                Kind::Send => send::run(interaction, data).await?,
                Kind::Repeat => repeat::run(interaction, data).await?,
                Kind::RepeatMessage => repeat_message::run(interaction, data).await?,
                Kind::Twee => twee::run(interaction, data).await?,
            }
        }
        _ => {}
    }

    Ok(())
}
