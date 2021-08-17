use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    name: Option<String>,
    url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Footer {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    name: String,
    value: String,
    inline: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    title: Option<String>,
    /**
    * rich: generic embed rendered from embed attributes
    * image: image embed
    * video: video embed
    * gifv: animated gif image embed rendered as a video embed
    * article: article embed
    * link: link embed
    */
    #[serde(rename="type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    footer: Option<Footer>,
    image: Option<Image>,
    thumbnail: Option<Thumbnail>,
    video: Option<Video>,
    provider: Option<Provider>,
    author: Option<Author>,
    fields: Option<Vec<Field>>
}
