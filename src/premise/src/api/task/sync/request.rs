use serde_json::Number;

use crate::api::{form_localization::survey::context, location::Location, task::Info};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub context_types: Vec<context::Type>,
    pub deleted_reservation_ids: Vec<Number>,
    pub local_state: Vec<LocalState>,
    #[serde(rename = "proxyLatLng")]
    pub location: Location,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalState {
    #[serde(rename = "syncItemType")]
    pub item_type: ItemType,
    #[serde(flatten)]
    pub task_info: Info,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ItemType {
    Reservation,
    #[serde(rename = "TASK_SUMMARY")]
    Summary,
    #[serde(rename = "TASK_CONFIG")]
    Configuration,
}

