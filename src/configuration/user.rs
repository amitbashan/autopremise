use premise::task::{self, Task};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
    pub refresh_token: String,
    pub location: premise::api::location::Location,
}

impl User {
    #[tracing::instrument(skip_all)]
    pub async fn automate(self) -> premise::result::Result<()> {
        let mut client = premise::client::Client::new(self.refresh_token, self.location).await?;

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
                    Ok(_) => {
                        tracing::info!(r#"Task "{}" submitted successfully."#, task.title);
                    }
                    Err(error) => {
                        tracing::error!(r#"Failed submitting "{}"."#, task.title);
                        tracing::debug!("{:#?}", error);
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(18000)).await;
            client.sync().await?;
        }
    }
}