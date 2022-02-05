#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Option {
    pub label: String,
    pub value: String,
}
