use chrono::{Duration, Utc};
use serde_json::Number;

use crate::api::number;

use super::{accuracy::Accuracy, Location, mode::Mode};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLocation {
	#[serde(flatten)]
	pub location: Location,
	#[serde(flatten)]
	pub accuracy: Accuracy,
	pub altitude: Number,
	pub speed: Number,
	pub bearing: Number,
	pub timestamp: number::Number,
	pub uptime_nanos: number::Number,
	pub system_location_mode: Option<Mode>,
	pub provider: String,
	pub from_mock_provider: bool,
}

impl UploadLocation {
	pub fn new(location: Location, uptime: Duration) -> Self {
		Self {
			location,
			accuracy: Accuracy::new(20, 40),
			altitude: Number::from(5),
			speed: Number::from(0),
			bearing: Number::from(90),
			timestamp: number::Number::from(Number::from(Utc::now().timestamp())),
			uptime_nanos: number::Number::from(Number::from(uptime.num_nanoseconds().unwrap())),
			system_location_mode: Some(Mode::HighAccuracy),
			provider: "fused".to_string(),
			from_mock_provider: false,
		}
	}
}
