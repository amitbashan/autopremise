use super::number::Number;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Campaign {
    #[serde(rename = "campaignId")]
    pub id: Number,
    #[serde(rename = "campaignVersion")]
    pub version: Number,
}