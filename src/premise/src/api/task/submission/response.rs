#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
	#[serde(flatten)]
	pub submission: super::request::Body,
	pub submitted_time: crate::api::time::Time,
}
