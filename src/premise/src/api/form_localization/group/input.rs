use serde_json::Number;
use serde_with::{NoneAsEmptyString, serde_as};

use super::super::relevance::Relevance;

#[serde_as]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputGroup {
    pub name: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub label: Option<String>,
    pub button_label: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub hint: Option<String>,
    pub inputs: Vec<Input>,
    pub repeat_at_least: Number,
    pub repeat_at_most: Number,
    #[serde(rename = "requiresManualQC")]
    pub requires_manual_qc: bool,
    pub invisible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance: Option<Relevance>,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub name: String,
    pub label: String,
    pub hint: Option<String>,
    pub necessity: Relevance,
    pub options: Option<Vec<crate::api::option::Option>>,
    pub style: Option<Style>,
    #[serde(rename = "inputType")]
    pub r#type: super::Type,
    pub class_name: String,
}

#[derive(Debug, Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Style {
    SingleLine,
    MultiLine,
    Dropdown,
    CheckboxGroup,
    Map,
}
