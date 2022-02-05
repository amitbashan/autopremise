use crate::make_endpoint;

pub mod request;
pub mod response;

pub const ENDPOINT: &str = make_endpoint!(v3/task/sync);

pub async fn sync(client: &reqwest::Client, body: &request::Body) -> reqwest::Result<response::Body> {
    client.post(ENDPOINT)
        .json(body)
        .send()
        .await?
        .json()
        .await
}
