use std::{
    sync::{LazyLock, Mutex, atomic::Ordering},
    time::{Duration, Instant},
};

use twilight_http::{
    Error, Response, request::channel::reaction::RequestReactionType, response::marker::EmptyBody,
};
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{
        Id,
        marker::{ChannelMarker, UserMarker},
    },
};

/// To avoid Discord rate limits, we are going to prevent username changes within 20 seconds.
static LAST_TWEE_RENAME: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now()));

const USERS_BRAINDAMAGE: [Id<UserMarker>; 7] = [
    USER_XENO,
    USER_FEROS,
    USER_STUPIDTOASTER,
    USER_ZEROLUX,
    USER_NIVA,
    USER_TWEEZERS,
    USER_CASEY,
];

const USERS_PLEADING: [Id<UserMarker>; 1] = [USER_YASHA];
const DEFAULT_WORD_LIMIT: usize = 2;
const DEFAULT_CHARACTER_LIMIT: usize = 16;
const FAKE_TOP: [&str; 3] = ["pussy", "dom", "top"];

pub const CHANNEL_WORK: Id<ChannelMarker> = Id::new(1534287648955306024);
pub const CHANNEL_VENT: Id<ChannelMarker> = Id::new(1139232506990833725);
pub const CHANNEL_POL: Id<ChannelMarker> = Id::new(1260223411024560218);

use crate::{
    core::{
        BOOP_COUNTER, CTX, EMOJI_BRAINDAMAGE, EMOJI_PLEADING, GUILD_COZY, USER_CASEY, USER_FEROS,
        USER_LISU, USER_NIVA, USER_STUPIDTOASTER, USER_TWEEZERS, USER_XENO, USER_YASHA,
        USER_ZEROLUX,
    },
    services::interaction_handler::twee::TWEE_NICKNAMES,
};

pub async fn message_handler(msg: Box<MessageCreate>) -> anyhow::Result<()> {
    tracing::info!("{}: {}", msg.author.name, msg.content);

    if msg.channel_id == CHANNEL_WORK
        || msg.channel_id == CHANNEL_VENT
        || msg.channel_id == CHANNEL_POL
    {
        return Ok(());
    }

    let (word_limit, character_limit) = if msg.guild_id.is_some_and(|x| x == GUILD_COZY) {
        let word_limit = vec![1, 2, 3, 4, 5, 6];
        let character_limit = vec![4, 6, 8, 12, 16, 32];

        (
            fastrand::usize(..word_limit.len()),
            fastrand::usize(..character_limit.len()),
        )
    } else {
        (DEFAULT_WORD_LIMIT, DEFAULT_CHARACTER_LIMIT)
    };

    let content_lower = msg.content.to_lowercase();

    if content_lower.trim() == "boop" {
        let new_count = BOOP_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        let alert = format!("Boop! 🐾 (Total boops administered: {new_count})");
        CTX.http
            .create_message(msg.channel_id)
            .content(&alert)
            .await?;
    }

    let slice = msg.content.as_str();

    if FAKE_TOP.iter().any(|&word| content_lower.contains(word)) && msg.author.id == USER_LISU {
        send_reaction(&msg, EMOJI_PLEADING).await?;
    }

    if msg.content.len() <= character_limit || slice.split_whitespace().count() <= word_limit {
        if USERS_PLEADING.contains(&msg.author.id) {
            send_reaction(&msg, EMOJI_PLEADING).await?;
        }

        if USERS_BRAINDAMAGE.contains(&msg.author.id) {
            send_reaction(&msg, EMOJI_BRAINDAMAGE).await?;
        }
    }

    if let Some(guild_id) = msg.guild_id
        && guild_id == GUILD_COZY
        && msg.author.id == USER_TWEEZERS
    {
        let should_rename = {
            let mut last_rename = LAST_TWEE_RENAME.lock().unwrap();
            if last_rename.elapsed() >= Duration::from_secs(20) {
                *last_rename = Instant::now();
                true
            } else {
                false
            }
        };

        if should_rename {
            let name = fastrand::choice(TWEE_NICKNAMES).unwrap_or(TWEE_NICKNAMES[0]);

            CTX.http
                .update_guild_member(GUILD_COZY, USER_TWEEZERS)
                .nick(name.into())
                .await?
                .model()
                .await?;
        }
    }

    Ok(())
}

/// A shorthand to create a reaction on the message passed, using the HTTP client rather than the interaction client.
async fn send_reaction<'a>(
    msg: &MessageCreate,
    reaction: RequestReactionType<'a>,
) -> Result<Response<EmptyBody>, Error> {
    CTX.http
        .create_reaction(msg.channel_id, msg.id, &reaction)
        .await
}
