use std::sync::{LazyLock, Mutex};

use twilight_model::{
    application::{
        command::{CommandOption, CommandType},
        interaction::application_command::CommandData,
    },
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::core::{CTX, InteractionService};

static NEW_NICKNAMES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

#[derive(Default)]
pub struct AddNickname;

impl InteractionService for AddNickname {
    const NAME: &str = "add_nickname";
    const DESCRIPTION: &str = "Add a nickname that can be randomly assigned to users.";

    fn command_def() -> twilight_model::application::command::Command {
        CommandBuilder::new(Self::NAME, Self::DESCRIPTION, CommandType::ChatInput)
            .option(CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: "The nickname that you want to add".to_owned(),
                description_localizations: None,
                kind: twilight_model::application::command::CommandOptionType::String,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: "nickname".to_owned(),
                name_localizations: None,
                options: None,
                required: Some(true),
            })
            .build()
    }

    async fn run(
        &self,
        interaction: Box<InteractionCreate>,
        data: Box<CommandData>,
    ) -> anyhow::Result<()> {
        let mut nick = String::from("AWAWAWAWAWA");
        let mut nicknames_saved = 0;

        // Robustly extract options by name to avoid index ordering issues
        for opt in &data.options {
            match opt.name.as_str() {
            "nickname" => {
                if let twilight_model::application::interaction::application_command::CommandOptionValue::String(val) = &opt.value {
                    nick = val.clone();
                }
            }
            _ => {}
        }
        }

        if let Ok(mut mutex) = NEW_NICKNAMES.lock() {
            mutex.push(nick.clone());
            nicknames_saved = mutex.len();
        }

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(format!(
                    "The nickname `{nick}` has been added!\nThank you for your service.\n-# There are now `{nicknames_saved}`'s in total.",
                )),
                ..Default::default()
            }),
        };

        CTX.interaction()
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }
}
