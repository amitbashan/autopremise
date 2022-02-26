use chrono::{Timelike, Utc};
use premise::task::{self, Task};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
	pub refresh_token: String,
	pub location: premise::api::location::Location,
	pub proxy: Option<String>,
	pub r#loop: bool,
}

impl User {
	#[tracing::instrument(skip_all)]
	pub async fn automate(self) -> premise::result::Result<premise::client::Client> {
		tracing::info!(r#"Creating client with token "{}"…"#, self.refresh_token);

		let client = premise::client::Client::new(
			self.refresh_token,
			self.location,
			self.proxy.map(reqwest::Proxy::all).transpose()?,
		).await?;

		tracing::info!("Automating user {} ({})…", client.user.data.id, client.user.data.email);

		loop {
			for task in &client.cache.tasks {
				if task.title == "Update your Product ID preferences"
					|| task.requirements.len() > 0
					|| task.requires_travel
					|| task.requires_photos
					|| task.requires_screenshots {
					continue;
				}

				tracing::info!(r#"Submitting "{}"."#, task.title);

				match task::survey::Survey(task.info.id.0.clone()).submit(&client).await {
					Ok(_) => tracing::info!(r#"Task "{}" submitted successfully."#, task.title),
					Err(error) => tracing::error!(r#"Failed submitting "{}". Error: {}."#, task.title, error.to_string()),
				}

				tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
			}

			if !self.r#loop {
				break;
			}

			let hours = (24 - Utc::now().hour() + 5) as u64;

			tracing::info!("User {} ({}) went to sleep for {} hours…", client.user.data.id, client.user.data.email, hours);
			tokio::time::sleep(
				tokio::time::Duration::from_secs(
					3600 * hours,
				)
			).await;
		}

		Ok(client)
	}
}