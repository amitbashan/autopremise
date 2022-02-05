use super::Token;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Body {
    pub access_token: Token,
    pub expires_in: String,
    pub token_type: String,
    pub refresh_token: Token,
    pub id_token: Token,
    pub user_id: String,
    pub project_id: String,
}