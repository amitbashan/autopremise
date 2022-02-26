use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Number(
	#[serde_as(as = "DisplayFromStr")]
	pub serde_json::Number,
);

impl TryFrom<&str> for Number {
	type Error = serde_json::Error;

	fn try_from(string: &str) -> Result<Self, Self::Error> {
		use std::str::FromStr;

		Ok(Number(serde_json::Number::from_str(&string)?))
	}
}

impl From<serde_json::Number> for Number {
	fn from(number: serde_json::Number) -> Self {
		Self(number)
	}
}