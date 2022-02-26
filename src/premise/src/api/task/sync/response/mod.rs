use serde_json::Number;

use crate::api::task::{Configuration, Task};

pub mod reservation;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
	pub allocation_metadata: AllocationMetadata,
	pub deleted_reservation_ids: Vec<String>,
	pub deleted_summary_ids: Vec<String>,
	pub reservations: Vec<reservation::Reservation>,
	#[serde(rename = "taskSummaries")]
	pub tasks: Vec<Task>,
	#[serde(rename = "taskConfigs")]
	pub task_configurations: Vec<Configuration>,
	pub debug_string: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationMetadata {
	pub reservation_limit: Number,
	pub reservations_remaining: Number,
}
