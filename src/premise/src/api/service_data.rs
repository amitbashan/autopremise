use super::number::Number;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ServiceData {
	#[serde(rename = "formId")]
	pub id: Number,
	#[serde(rename = "formLocalizationId")]
	pub localization_id: Number,
	#[serde(rename = "formVersion")]
	pub version: Number,
	#[serde(rename = "formLocalizationVersion")]
	pub localization_version: Number,
}
