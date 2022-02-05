#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body<'a> {
    pub grant_type: &'static str,
    #[serde(rename = "refreshToken")]
    pub token: &'a str,
}
