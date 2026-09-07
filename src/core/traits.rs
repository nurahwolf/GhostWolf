use twilight_model::{
    application::{command::Command, interaction::application_command::CommandData},
    gateway::payload::incoming::InteractionCreate,
};

/// This allows a service to handle interactions.
pub trait InteractionService: Send + Sync + Sized {
    /// This is the name of the interaction
    const NAME: &str;
    /// This is the description of the interaction
    const DESCRIPTION: &str;

    fn command_def() -> Command;
    async fn run(
        &self,
        interaction: Box<InteractionCreate>,
        data: Box<CommandData>,
    ) -> anyhow::Result<()>;
    // ... autocomplete
}
