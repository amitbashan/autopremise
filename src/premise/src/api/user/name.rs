#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Name {
	#[serde(rename = "firstName")]
	pub first: String,
	#[serde(rename = "lastName")]
	pub last: String,
	#[serde(rename = "fullName")]
	pub full: Option<String>,
}
