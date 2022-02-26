use rand::{distributions::{Distribution, Standard}, Rng};
use serde_json::Number;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Accuracy {
	#[serde(rename = "accuracy")]
	pub horizontal: Number,
	#[serde(rename = "verticalAccuracy")]
	pub vertical: Number,
}

impl Accuracy {
	pub fn new(horizontal: usize, vertical: usize) -> Self {
		let [horizontal, vertical] = [horizontal, vertical]
			.map(|number| Number::from(number));

		Self {
			horizontal,
			vertical,
		}
	}
}

impl Distribution<Accuracy> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Accuracy {
		let [horizontal, vertical] = [rng.gen_range(10..80), rng.gen_range(40..80)]
			.map(|number| Number::from(number));

		Accuracy {
			horizontal,
			vertical,
		}
	}
}
