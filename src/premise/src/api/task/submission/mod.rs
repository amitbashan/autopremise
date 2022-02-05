use crate::make_endpoint;

pub mod request;
pub mod response;

const ENDPOINT: &str = make_endpoint!(v2/submissions);

pub async fn submit(client: &reqwest::Client, body: &request::Body) -> reqwest::Result<response::Body> {
    client.post(ENDPOINT)
        .json(body)
        .send()
        .await?
        .json()
        .await
}
