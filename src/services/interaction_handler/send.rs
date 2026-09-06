use anyhow::Context;
use reqwest::Client;
use serde_json::json;
use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType},
        interaction::application_command::CommandData,
    },
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::core::CTX;

pub const NAME: &str = "send";

pub fn command() -> Command {
    CommandBuilder::new(NAME, "Send a message (as a user)", CommandType::ChatInput)
        .option(CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: "The message to send".to_owned(),
            description_localizations: None,
            kind: twilight_model::application::command::CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "msg".to_owned(),
            name_localizations: None,
            options: None,
            required: None,
        })
        .build()
}

pub async fn autocomplete(
    _interaction: Box<InteractionCreate>,
    _data: Box<CommandData>,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn run(
    interaction: Box<InteractionCreate>,
    data: Box<CommandData>,
) -> anyhow::Result<()> {
    let user_token = std::env::var("USER_TOKEN").context("reading `USER_TOKEN`")?;

    let http_client = Client::new();

    // Discord API v10 endpoint for creating a message
    let url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        interaction.channel.clone().unwrap().id
    );

    if let Some(message) = data.options.first() {
        let message = match message.value.clone() {
	        twilight_model::application::interaction::application_command::CommandOptionValue::String(msg) => msg,
	        _ => "AWAWAWAWAWA".to_owned()
     	};

        // Make the POST request
        let response = http_client
            .post(&url)
            // Crucial: Pass the raw token WITHOUT the "Bot " prefix
            .header("Authorization", user_token)
            .header("Content-Type", "application/json")
            // User-Agent is sometimes checked by Discord's anti-abuse systems
            .header("User-Agent", "curl/7.54.1")
            .json(&json!({
                "content": message,
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            println!("Message sent successfully!");
        } else {
            eprintln!("Failed to send message. Status: {}. Body: {}", status, body);
        }

        let data = InteractionResponseData {
            content: Some("Message Proxied!".to_owned()),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        };

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        };
        CTX.interaction()
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        return Ok(());
    }

    Ok(())
}
