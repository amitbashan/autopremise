use serde_json::Number;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EstimatedDuration {
	#[serde(rename = "lowerBoundsSeconds", alias = "lowerBoundSeconds")]
	pub lower_bound: Number,
	#[serde(rename = "upperBoundsSeconds", alias = "upperBoundSeconds")]
	pub upper_bound: Number,
}
