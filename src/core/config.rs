/// This controlls the types of events the bot listens to.
pub const CONFIG_EVENT_TYPES: twilight_gateway::EventTypeFlags =
    twilight_gateway::EventTypeFlags::all();

/// This controlls the intent types that are requested from Discord.
pub const CONFIG_INTENTS: twilight_gateway::Intents = twilight_gateway::Intents::all();
