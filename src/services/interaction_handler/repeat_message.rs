use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandData,
    },
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

pub const NAME: &str = "repeat_message";

pub fn command() -> Command {
    CommandBuilder::new(NAME, "", CommandType::Message).build()
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
    // 1. Extract the target message ID from the interaction data
    let target_id = data
        .target_id
        .ok_or_else(|| anyhow::anyhow!("Interaction missing target ID"))?
        .cast();

    // 2. Extract the resolved messages map
    let resolved_messages = data
        .resolved
        .map(|x| x.messages)
        .ok_or(anyhow::anyhow!("Interaction missing target ID"))?;

    // Note: Depending on your exact twilight version, target_id might need to be
    // cast to Id<MessageMarker>. If it complains about types, use:
    // let msg_id = Id::<MessageMarker>::new(target_id.get());
    let target_msg = resolved_messages
        .get(&target_id)
        .ok_or_else(|| anyhow::anyhow!("Target message not found in resolved data"))?;

    // 3. Get the message content.
    // We check for empty content in case the user right-clicked an image-only message.
    let msg = if target_msg.content.is_empty() {
        // Fallback to prevent Discord from rejecting an empty message payload
        "[Repeated message had no text content]".to_owned()
    } else {
        target_msg.content.clone()
    };

    // 5. Send the first message as the initial interaction response
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

    Ok(())
}
