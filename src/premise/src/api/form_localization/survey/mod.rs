pub mod context;
pub mod locale;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Survey {
	pub contexts: Vec<context::Context>,
}
