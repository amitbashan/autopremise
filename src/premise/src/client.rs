use chrono::{Duration, Utc};

use crate::{
    api::{
        location::Location,
        task,
        time::Time,
    },
    result,
    user::User,
};

#[derive(Debug)]
pub struct Client {
    pub user: User,
    pub creation_time: Time,
    pub cache: task::sync::response::Body,
}

impl Client {
    pub async fn new(token: String, location: Location, proxy: Option<reqwest::Proxy>) -> result::Result<Self> {
        let user = User::from_refresh_token(token, location, proxy).await?;
        let cache = user.sync().await?;

        Ok(
            Self {
                user,
                creation_time: Utc::now(),
                cache,
            }
        )
    }

    pub async fn sync(&mut self) -> result::Result<()> {
        self.cache = self.user.sync().await?;

        Ok(())
    }

    pub fn uptime(&self) -> Duration {
        Utc::now() - self.creation_time
    }
}
