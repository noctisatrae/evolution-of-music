use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub collaborative: bool,
    pub description: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: Owner,
    #[serde(rename = "primary_color")]
    pub primary_color: Value,
    pub public: bool,
    #[serde(rename = "snapshot_id")]
    pub snapshot_id: String,
    pub tracks: Tracks,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub href: Value,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub height: Value,
    pub url: String,
    pub width: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls2,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls2 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracks {
    pub href: String,
    pub items: Vec<Item>,
    pub limit: i64,
    pub next: Value,
    pub offset: i64,
    pub previous: Value,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "added_at")]
    pub added_at: String,
    #[serde(rename = "added_by")]
    pub added_by: AddedBy,
    #[serde(rename = "is_local")]
    pub is_local: bool,
    #[serde(rename = "primary_color")]
    pub primary_color: Value,
    pub track: Track,
    #[serde(rename = "video_thumbnail")]
    pub video_thumbnail: VideoThumbnail,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddedBy {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls3,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls3 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist2>,
    #[serde(rename = "disc_number")]
    pub disc_number: i64,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    pub episode: bool,
    pub explicit: bool,
    #[serde(rename = "external_ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls7,
    pub href: String,
    pub id: String,
    #[serde(rename = "is_local")]
    pub is_local: bool,
    #[serde(rename = "is_playable")]
    pub is_playable: bool,
    pub name: String,
    /// The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.
    pub popularity: i64,
    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,
    pub track: bool,
    #[serde(rename = "track_number")]
    pub track_number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    #[serde(rename = "linked_from")]
    pub linked_from: Option<LinkedFrom>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "album_group")]
    pub album_group: String,
    #[serde(rename = "album_type")]
    pub album_type: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls5,
    pub href: String,
    pub id: String,
    pub images: Vec<Image2>,
    #[serde(rename = "is_playable")]
    pub is_playable: bool,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls4,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls4 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls5 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist2 {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls6,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls6 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls7 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFrom {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls8,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls8 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoThumbnail {
    pub url: Value,
}

// ==============================================================================
#[derive(Serialize, Deserialize, Debug)]
pub struct Cleaned {
    pub name: String,
    pub duration_ms: i64,
    /// The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.
    pub popularity: i64,
    pub id: String,
    pub audio_feature: AudioAnalysis,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioAnalysis {
    pub acousticness: f64,
    #[serde(rename = "analysis_url")]
    pub analysis_url: String,
    pub danceability: f64,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    pub energy: f64,
    pub id: String,
    pub instrumentalness: f64,
    pub key: i64,
    pub liveness: f64,
    pub loudness: f64,
    pub mode: i64,
    pub speechiness: f64,
    pub tempo: f64,
    #[serde(rename = "time_signature")]
    pub time_signature: i64,
    #[serde(rename = "track_href")]
    pub track_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub valence: f64,
}
