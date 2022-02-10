pub mod input;
pub mod output;

#[derive(Debug, Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Number,
    Text,
    Photo,
    Screenshot,
    #[serde(rename = "GEOPOINT")]
    GeoPoint,
    SelectOne,
    SelectMany,
    Scanner,
}
