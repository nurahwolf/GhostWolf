use std::sync::atomic::Ordering;

use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{Id, marker::UserMarker},
};

const TARGETS: [Id<UserMarker>; 5] = [
    USER_XENO,
    USER_FEROS,
    Id::<UserMarker>::new(1283928196609474560),
    Id::<UserMarker>::new(349235480987959308),
    USER_NIVA,
];

const SUBS: [Id<UserMarker>; 1] = [USER_YASHA];
const DEFAULT_WORD_LIMIT: usize = 2;
const DEFAULT_CHARACTER_LIMIT: usize = 16;
const FAKE_TOP: [&str; 3] = ["pussy", "dom", "top"];

use crate::{
    core::{
        BOOP_COUNTER, CTX, EMOJI_BRAINDAMAGE, EMOJI_PLEADING, GUILD_COZY, USER_CASEY, USER_FEROS,
        USER_LISU, USER_NIVA, USER_TWEEZERS, USER_XENO, USER_YASHA,
    },
    services::interaction_handler::twee::TWEE_NICKNAMES,
};

pub async fn message_handler(msg: Box<MessageCreate>) -> anyhow::Result<()> {
    tracing::info!("{}: {}", msg.author.name, msg.content);

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

    if FAKE_TOP.iter().any(|&word| msg.content.contains(word)) && msg.author.id == USER_LISU {
        send_reaction(&msg, EMOJI_PLEADING).await?;
    }

    if msg.content.len() <= character_limit || slice.split_whitespace().count() <= word_limit {
        if SUBS.contains(&msg.author.id) {
            send_reaction(&msg, EMOJI_PLEADING).await?;
        }

        if TARGETS.contains(&msg.author.id) {
            send_reaction(&msg, EMOJI_BRAINDAMAGE).await?;
        }

        if msg.author.id == USER_TWEEZERS || msg.author.id == USER_CASEY {
            send_reaction(&msg, EMOJI_BRAINDAMAGE).await?;
        }
    }

    // Also update her nickname whenever she posts in my server
    if let Some(guild_id) = msg.guild_id
        && guild_id == GUILD_COZY
    {
        let name = fastrand::choice(TWEE_NICKNAMES).unwrap_or(TWEE_NICKNAMES[0]);

        CTX.http
            .update_guild_member(GUILD_COZY, USER_TWEEZERS)
            .nick(name.into())
            .await?
            .model()
            .await?;
    }

    // if content_lower.contains("yiff") || content_lower.contains("http") {
    //     let alert = "
    //     🛡️ **[SECURITY ALERT]** Nurah’s Intrusion Prevention System (NIPS) has flagged this packet. \n
    //     **Threat level:** *Silly*.\n
    //     Please remain hydrated and keep your paws off the keyboard.";

    //     CTX.http
    //         .create_message(msg.channel_id)
    //         .content(alert)
    //         .await?;
    // }

    Ok(())
}

async fn send_reaction<'a>(
    msg: &MessageCreate,
    reaction: RequestReactionType<'a>,
) -> anyhow::Result<()> {
    CTX.http
        .create_reaction(msg.channel_id, msg.id, &reaction)
        .await?;

    Ok(())
}
