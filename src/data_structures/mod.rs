use serde::{Serialize, Deserialize};
use embed::Embed;

pub mod embed;
pub mod sticker;

pub type Snowflake = i64;

#[derive(Serialize, Deserialize, Debug)]
#[repr(u32)]
enum ApplicationFlag {
    GatewayPresence=1<<12,
    GatewayPresenceLimited=1<<13,
    GatewayGuildMembers=1<<14,
    GatewayGuildMembersLimited=1<<15,
    VerificationPendingGuildLimit=1<<16,
    Embedded=1<<17
}

#[derive(Serialize, Deserialize, Debug)]
struct Application {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_services_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<User>,
    summary: String,
    verify_key: String,
    team: Option<Team>,
    guild_id: Option<Snowflake>,
    primary_sku_id: Option<Snowflake>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeamMember {
    membership_state: u8,
    permissions: Vec<String>,
    team_id: Snowflake,
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    icon: Option<String>,
    id: Snowflake,
    members: Vec<TeamMember>,
    name: String,
    owner_user_id: Snowflake,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u32)]
enum UserFlags {
    None=0,
    DiscordEmployee=1<<0,
    PartneredOwner=1<<1,
    HypeSquadEvents=1<<2,
    BugHunterLvl1=1<<3,
    HouseBravery=1<<6,
    HouseBrilliance=1<<7,
    HouseBalance=1<<8,
    EarlySupporter=1<<9,
    TeamUser=1<<10,
    BugHunterLvl2=1<<14,
    VerifiedBot=1<<16,
    EarlyVerifiedBotDev=1<<17,
    DiscordCerifiedMod=1<<18
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum PremiumTypes {
    None=0,
    NitroClassic=1,
    Nitro=2
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum VideoQuality {
    Auto=1,
    Full=2
}


#[derive(Serialize, Deserialize, Debug)]
struct Overwrite {
    id: Snowflake,
    #[serde(rename="type")]
    overwrite_type: u8,
    allow: String,
    deny: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u32>,
    premium_type: Option<u8>,
    public_flags: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct RoleTags {
    bot_id: Option<Snowflake>,
    integration_id: Option<Snowflake>,
    premium_subscriber: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Role {
    id: Snowflake,
    name: String,
    color: u32,
    hoist: bool,
    position: u32,
    // Permission bit set, so shouldn't this be an integer?
    permissions: String,
    managed: bool,
    mentionable: bool,
    tags: RoleTags,
}

#[derive(Serialize, Deserialize, Debug)]
struct ThreadMeta {
    archived: bool,
    auto_archive_duration: u32,
    archive_timestamp: String,
    locked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ThreadMember {
    id: Option<Snowflake>,
    user_id: Option<Snowflake>,
    join_timestamp: String,
    flags: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<Snowflake>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    #[serde(rename="type")]
    channel_type: u8,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum ChannelTypes {
    GuildText=0,
    DM=1,
    GuildVoice=2,
    GroupDM=3,
    GuildCategory=4,
    GuildNews=5,
    GuildStore=6,
    GuildNewsThread=10,
    GuildPublicThread=11,
    GuildPrivateThread=12,
    GuildStageVoice=13,
}

#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    id: Snowflake,
    #[serde(rename="type")]
    channel_type: u8,
    guild_id: Option<Snowflake>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
    last_pin_timestamp: Option<String>,
    rtc_region: Option<String>,
    video_quality_mode: Option<u8>,
    message_count: Option<u32>,
    member_count: Option<u32>,
    thread_metadata: Option<ThreadMeta>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: Option<u32>,
    permisions: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Attachment {
    id: Snowflake,
    filename: String,
    content_type: Option<String>,
    size: u32,
    url: String,
    proxy_url: String,
    height: Option<u32>,
    width: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    id: Option<Snowflake>,
    name: Option<Snowflake>,
    roles: Option<Vec<Snowflake>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
struct Reaction {
    count: u32,
    me: bool,
    emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum ActivityType {
    Join=1,
    Spectate=2,
    Listen=3,
    JoinRequest=4
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageActivity {
    #[serde(rename="type")]
    activity_type: u8,
    party_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum MessageFlag {
    Crossposted=1<<0,
    IsCrosspost=1<<1,
    SupressEmbeds=1<<2,
    SourceMessageDeleted=1<<3,
    Urgent=1<<4,
    HasThread=1<<5,
    Ephemeral=1<<6,
    Loading=1<<7
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
enum InteractionType {
    Ping=1,
    ApplicationCommand=2,
    MessageComponent=3,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageInteraction {
    id: Snowflake,
    #[serde(rename="type")]
    interaction_type: u8,
    name: String,
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageReference {
    message_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    guild_id: Option<Snowflake>,
    fail_if_not_exists: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectOption {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<Emoji>,
    default: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
struct Component {
    #[serde(rename="type")]
    component_type: u8,
    custom_id: Option<String>,
    disabled: Option<bool>,
    style: Option<u32>,
    label: Option<String>,
    emoji: Option<Emoji>,
    url: Option<String>,
    options: Vec<SelectOption>,
    placeholder: Option<String>,
    min_values: Option<u32>,
    max_values: Option<u32>,
    components: Option<Vec<Box<Component>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    guild_id: Option<Snowflake>,
    author: Option<User>,
    member: Option<GuildMember>,
    content: String,
    timestamp: String,
    edited_timestamp: String,
    tts: bool,
    mention_everyone: bool,
    // TODO: Handle mentions, array of user objects, with an additional partial member field
    mentions: String,
    mention_roles: Vec<Snowflake>,
    mention_channels: Vec<ChannelMention>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    #[serde(rename="type")]
    message_type: u32,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<Snowflake>,
    message_reference: Option<MessageReference>,
    flags: Option<u8>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    components: Option<Vec<Component>>,
    sticker_items: Option<Vec<sticker::StickerItem>>,
    stickers: Option<Vec<sticker::Sticker>>,
}
