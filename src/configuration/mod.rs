pub mod user;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Configuration {
	pub users: Vec<user::User>,
}
