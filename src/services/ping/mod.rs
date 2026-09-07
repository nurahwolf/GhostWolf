use twilight_model::{
    application::{command::CommandType, interaction::application_command::CommandData},
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::core::InteractionService;

#[derive(Default)]
pub struct Ping;

impl InteractionService for Ping {
    const NAME: &str = "ping";
    const DESCRIPTION: &str = "Ping the bot";

    fn command_def() -> twilight_model::application::command::Command {
        CommandBuilder::new(Self::NAME, Self::DESCRIPTION, CommandType::ChatInput).build()
    }

    async fn run(
        &self,
        interaction: Box<InteractionCreate>,
        _data: Box<CommandData>,
    ) -> anyhow::Result<()> {
        let data = InteractionResponseData {
            content: Some("Pong!".to_owned()),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        };

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        };

        crate::core::CTX
            .interaction()
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }
}
