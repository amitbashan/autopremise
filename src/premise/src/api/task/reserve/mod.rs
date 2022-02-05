use serde_json::{json, Number};

use crate::{api::task::sync::response::reservation, make_endpoint};

pub mod request;
pub mod response;

pub async fn reserve(client: &reqwest::Client, body: &request::Body) -> reqwest::Result<response::Body> {
    const ENDPOINT: &str = make_endpoint!(v2/tasks/reserve);

    client.post(ENDPOINT)
        .json(body)
        .send()
        .await?
        .json()
        .await
}

pub async fn sync(client: &reqwest::Client, info: Vec<reservation::Info>) -> reqwest::Result<reqwest::Response> {
    const ENDPOINT: &str = make_endpoint!(v2/reservations/sync);

    client.put(ENDPOINT)
        .json(&json!({
			"deletedReservationIds": Vec::<Number>::new(),
			"reservations": info,
		}))
        .send()
        .await
}