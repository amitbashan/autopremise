use serde_json::Number;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Body(pub Vec<Number>);
