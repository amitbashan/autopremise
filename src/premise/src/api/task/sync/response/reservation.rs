use serde_json::Number;
use serde_with::{DisplayFromStr, serde_as};

use crate::api::{number, task, time::Time};

#[serde_as]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reservation {
	#[serde_as(as = "DisplayFromStr")]
	pub id: Number,
	#[serde_as(as = "DisplayFromStr")]
	pub user_id: Number,
	#[serde(flatten)]
	pub info: task::Info,
	#[serde(rename = "reservationTime")]
	pub time: Time,
	pub expiration_time: Time,
	#[serde(rename = "reservationPolicy")]
	pub policy: Policy,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
	#[serde(flatten)]
	pub task_info: task::Info,
	pub reservation_id: number::Number,
}

#[derive(Debug, Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Policy {
	Automatic,
	Manual,
}
