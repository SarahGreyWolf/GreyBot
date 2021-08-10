
type Snowflake = i64;

#[derive(Serialize, Deserialize, Debug)]
#[repr(u16)]
enum UserFlags {
    None=0,
    DiscordEmployee=1<<0,
    PartneredOwner=1<<1,
    HypeSquadEvents=1<<2,
    BugHunterLvl1=1<<3,
    HouseBravery=1<<6,
    HouseBrilliance=1<<7,
    HouseBalance=1<<8,
    EarlySupporter=1<<9
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
struct Overwrite {
    id: Snowflake,
    type: u8,
    allow: String,
    deny: String
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
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
    flags: Option<u16>,
    premium_type: Option<u8>,
    public_flags: Option<u16>
}

#[derive(Serialize, Deserialize, Debug)]
struct ThreadMeta {
    archived: bool,
    auto_archive_duration: u16,
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
struct Channel {
    id: Snowflake,
    #[serde(rename="type")]
    channel_type: u8,
    guild_id: Option<Snowflake>,
    position: Option<u16>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<u16>,
    user_limit: Option<u16>,
    rate_limit_per_user: Option<u16>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
    last_pin_timestamp: Option<String>,
    rtc_region: Option<String>,
    video_quality_mode: Option<u8>,
    message_count: Option<u16>,
    member_count: Option<u16>,
    thread_metadata: Option<ThreadMeta>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: Option<u16>,
    permisions: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    
}
