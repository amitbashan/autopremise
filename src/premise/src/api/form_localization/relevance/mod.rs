use serde_with::{NoneAsEmptyString, serde_as};

pub mod predicate;

#[serde_as]
#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relevance {
    pub predicate: predicate::Predicate,
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub message: Option<String>,
}
