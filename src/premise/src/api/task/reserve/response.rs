use crate::api::task::{Configuration, sync::response::reservation::Reservation};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
	pub reserved_tasks: Vec<ReservedTask>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReservedTask {
	pub reservation: Reservation,
	pub task: Configuration,
}
