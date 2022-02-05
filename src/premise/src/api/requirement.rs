#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Requirement {
    Photo,
    Screenshot,
}
