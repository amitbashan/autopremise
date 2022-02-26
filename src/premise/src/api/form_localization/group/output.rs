use crate::api::{location::upload::UploadLocation, time::Time};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputGroup {
	pub name: String,
	pub results: Vec<Result>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
	pub start_time: Time,
	pub end_time: Time,
	pub outputs: Vec<Output>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
	pub name: String,
	pub created_time: Time,
	pub location: UploadLocation,
	#[serde(with = "either::serde_untagged")]
	pub value: Value,
	#[serde(rename = "outputType")]
	pub r#type: super::Type,
	pub class_name: String,
}

pub type Value = either::Either<String, Vec<String>>;
