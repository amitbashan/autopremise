use async_trait::async_trait;

use crate::{client::Client, result::Result, task::generator::Generator};

pub mod survey;
pub mod generator;

#[async_trait]
pub trait Task<I>: Generator<I> {
    async fn submit(&self, client: &Client) -> Result<()>;

    fn id(&self) -> &serde_json::Number;
}
