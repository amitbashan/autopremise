use serde_json::Number;
use serde_with::{DisplayFromStr, serde_as};

use super::{
	campaign,
	estimated_duration::EstimatedDuration,
	form_localization,
	image::Thumbnail,
	monitoring,
	number,
	price,
	price::Price,
	requirement,
	service_data,
	time::Time,
};

pub mod sync;
pub mod reserve;
pub mod submission;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
	#[serde(flatten)]
	pub info: Info,
	pub category_name: String,
	#[serde(flatten)]
	pub price_amount: price::amount::Amount,
	pub tier: String,
	#[serde(rename = "taskThumbnailImage")]
	pub thumbnail_image: Thumbnail,
	pub title: String,
	pub r#type: form_localization::survey::context::Type,
	pub summary: String,
	pub rich_summary: Option<String>,
	pub projected_expiration_time: Time,
	pub estimated_duration: EstimatedDuration,
	pub requires_travel: bool,
	pub requires_photos: bool,
	pub requires_screenshots: bool,
	pub requirements: Vec<requirement::Requirement>,
	pub tags: Vec<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
	#[serde_as(as = "DisplayFromStr")]
	pub id: Number,
	#[serde_as(as = "DisplayFromStr")]
	pub version: Number,
	pub title: String,
	pub submission_price: Price,
	pub observation_price: Price,
	pub form_localization: form_localization::FormLocalization,
	#[serde(rename = "taskMonitoring")]
	pub monitoring: monitoring::Monitoring,
	#[serde(flatten)]
	pub campaign: campaign::Campaign,
	pub service_data: service_data::ServiceData,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Info {
	#[serde(rename = "taskId")]
	pub id: number::Number,
	#[serde(rename = "taskVersion")]
	pub version: number::Number,
}
