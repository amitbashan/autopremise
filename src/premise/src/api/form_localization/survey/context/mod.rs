use crate::api::form_localization::group;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
	#[serde(rename = "contextType", skip_deserializing)]
	pub r#type: Type,
	pub input_groups: Vec<group::input::InputGroup>,
	pub class_name: String,
}

#[derive(Debug, Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
	Empty,
	Route,
	Area,
}

impl Default for Type {
	fn default() -> Self {
		Self::Empty
	}
}
