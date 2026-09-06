use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::id::{Id, marker::EmojiMarker};

pub const EMOJI_PLEADING: RequestReactionType = RequestReactionType::Custom {
    id: Id::<EmojiMarker>::new(1545052833408098346),
    name: Some("pleading"),
};

pub const EMOJI_BRAINDAMAGE: RequestReactionType = RequestReactionType::Custom {
    id: Id::<EmojiMarker>::new(1544376649855017072),
    name: Some("brain_damage"),
};
