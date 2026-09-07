use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandData,
    },
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::core::{CTX, GUILD_COZY, USER_TWEEZERS};

pub const NAME: &str = "twee";
pub const TWEE_NICKNAMES: [&str; 21] = [
    "3'7 Dildo",
    "Biting Slut",
    "FUCK ME, KNOT ME, BREED ME",
    "I cum if bullied",
    "I just wanna cum!!!",
    "I topped a feather",
    "I'm horny (and tiny).",
    "Itty Bitty Kitty Titties",
    "My Pussy Is For Breeding",
    "Non-Con is hot",
    "Please Abuse My Tiny Body",
    "Please Cum Inside Me",
    "S'mitties",
    "Short Snacc",
    "Smittens",
    "Smol and FUCKABLE",
    "Tiny Tits",
    "Fox Knot Socket",
    "Knot Socket",
    "Werewolf Knot Socket",
    "Mooda's Knot Socket",
];

pub fn command() -> Command {
    CommandBuilder::new(NAME, "Correct Tweezer's name", CommandType::ChatInput).build()
}

pub async fn autocomplete(
    _interaction: Box<InteractionCreate>,
    _data: Box<CommandData>,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn run(
    interaction: Box<InteractionCreate>,
    _data: Box<CommandData>,
) -> anyhow::Result<()> {
    let name = fastrand::choice(TWEE_NICKNAMES).unwrap_or(TWEE_NICKNAMES[0]);

    let data = InteractionResponseData {
        content: Some(format!(
            "Twee's name has been corrected to `{name}`!\nThank you for your service.",
        )),
        ..Default::default()
    };

    let name = if name.len() <= 1 { None } else { Some(name) };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };
    CTX.interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    CTX.http
        .update_guild_member(GUILD_COZY, USER_TWEEZERS)
        .nick(name.into())
        .await?
        .model()
        .await?;

    Ok(())
}
