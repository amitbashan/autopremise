use log::{error, info};
use premise::task::{self, Task};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
    pub refresh_token: String,
    pub location: premise::api::location::Location,
}

impl User {
    pub async fn automate(self) -> premise::result::Result<()> {
        let client = premise::client::Client::new(self.refresh_token, self.location).await?;

        loop {
            if client.cache.tasks.is_empty() {
                break;
            }

            for task in &client.cache.tasks {
                if task.title == "Update your Product ID preferences"
                    || task.requirements.len() > 0
                    || task.requires_travel
                    || task.requires_photos
                    || task.requires_screenshots {
                    continue;
                }

                match task::survey::Survey(task.info.id.0.clone()).submit(&client).await {
                    Ok(_) => {
                        info!(r#"Task "{}" submitted successfully."#, task.title);
                    }
                    Err(error) => {
                        error!(r#"Failed submitting "{}": {:#?}"#, task.title, error);
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }

            client.user.sync().await?;
        }

        Ok(())
    }
}