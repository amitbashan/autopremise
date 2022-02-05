use async_trait::async_trait;
use serde_json::{json, Number};

use crate::api::{update::{self, Update}, user::Data};

pub mod upload;
pub mod accuracy;
pub mod mode;

#[derive(Debug, PartialEq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Location {
    pub latitude: Number,
    pub longitude: Number,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        let [latitude, longitude] = [latitude, longitude]
            .map(|number| Number::from_f64(number)
                .expect("Latitude or longitude is either infinite or not a number."));

        Self {
            latitude,
            longitude,
        }
    }
}

#[async_trait]
impl Update for Location {
    async fn update(self, client: &reqwest::Client) -> reqwest::Result<Data> {
        let body = json!({
			"location": self,
		});

        client
            .post(update::ENDPOINT)
            .json(&body)
            .send()
            .await?
            .json()
            .await
    }
}