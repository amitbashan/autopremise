use url::Url;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Thumbnail {
	pub url: Url,
	pub google_image_services_url: Option<Url>,
}
