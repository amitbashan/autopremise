use reqwest::header::{HeaderValue, InvalidHeaderValue};

pub mod request;
pub mod response;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Token(String);

impl TryFrom<&Token> for HeaderValue {
    type Error = InvalidHeaderValue;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let mut header = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.0))?;

        header.set_sensitive(true);

        Ok(header)
    }
}
