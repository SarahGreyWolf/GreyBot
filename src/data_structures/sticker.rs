use serde::{Serialize, Deserialize};
use crate::data_structures::{Snowflake, User};

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum Type {
    Standard=1,
    Guild=2
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum FormatType {
    PNG=1,
    APNG=2,
    Lottie=3
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerItem {
    id: Snowflake,
    name: String,
    format_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Option<Snowflake>,
    name: String,
    description: Option<String>,
    tags: String,
    asset: String,
    #[serde(rename="type")]
    sticker_type: u32,
    format_type: u32,
    available: Option<bool>,
    guild_id: Option<Snowflake>,
    user: Option<User>,
    sort_value: Option<u32>
}
