use async_trait::async_trait;

use crate::make_endpoint;

use super::user::Data;

pub const ENDPOINT: &str = make_endpoint!(v2/users/update);

#[async_trait]
pub trait Update {
	async fn update(self, client: &reqwest::Client) -> reqwest::Result<Data>;
}
