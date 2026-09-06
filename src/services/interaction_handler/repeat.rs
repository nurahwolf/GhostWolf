use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType},
        interaction::application_command::CommandData,
    },
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

pub const NAME: &str = "repeat";

fn msg() -> CommandOption {
    CommandOption {
        autocomplete: None,
        channel_types: None,
        choices: None,
        description: String::from("What to say"),
        description_localizations: None,
        kind: twilight_model::application::command::CommandOptionType::String,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "msg".to_owned(),
        name_localizations: None,
        options: None,
        required: Some(true),
    }
}

fn count() -> CommandOption {
    CommandOption {
        autocomplete: None,
        channel_types: None,
        choices: None,
        description: "How many times".to_owned(),
        description_localizations: None,
        kind: twilight_model::application::command::CommandOptionType::Integer,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "count".to_owned(),
        name_localizations: None,
        options: None,
        required: Some(false),
    }
}

pub fn command() -> Command {
    CommandBuilder::new(NAME, "Repeat a message", CommandType::ChatInput)
        .option(msg())
        .option(count())
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
    let mut msg = String::from("AWAWAWAWAWA");
    let mut count: i64 = 1;

    // Robustly extract options by name to avoid index ordering issues
    for opt in &data.options {
        match opt.name.as_str() {
            "msg" => {
                if let twilight_model::application::interaction::application_command::CommandOptionValue::String(val) = &opt.value {
                    msg = val.clone();
                }
            }
            "count" => {
                if let twilight_model::application::interaction::application_command::CommandOptionValue::Integer(val) = &opt.value {
                    count = *val;
                }
            }
            _ => {}
        }
    }

    // Security clamp: Discord rate limits message creation to 5 per 5 seconds.
    // Capping at 5 prevents the bot from getting rate-limited or flagged for spam.
    let count = count.clamp(1, 5) as usize;

    // 1. Send the first message as the initial interaction response
    let response_data = InteractionResponseData {
        content: Some(msg.clone()),
        ..Default::default()
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(response_data),
    };

    crate::core::CTX
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    // 2. Send the remaining messages via the HTTP client
    if let Some(channel) = &interaction.channel {
        for _ in 1..count {
            // Note: Assuming CTX is a wrapper with a .http() method.
            // If CTX is the raw twilight_http::Client, just use CTX.create_message(...)
            crate::core::CTX
                .http
                .create_message(channel.id)
                .content(&msg)
                .await?;
        }
    }

    Ok(())
}
