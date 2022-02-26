use serde_json::Number;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Amount {
	#[serde(rename = "minPriceAmount")]
	#[serde_as(as = "DisplayFromStr")]
	pub minimum: Number,
	#[serde(rename = "maxPriceAmount")]
	#[serde_as(as = "DisplayFromStr")]
	pub maximum: Number,
	#[serde(rename = "maxPriceCurrency")]
	pub currency: crate::api::currency::Currency,
}
