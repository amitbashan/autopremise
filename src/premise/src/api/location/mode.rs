#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
	HighAccuracy,
}
