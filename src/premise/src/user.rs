use reqwest::header::{self, HeaderValue};
use serde_json::Number;

use crate::{
    api::{
        form_localization::survey::context,
        google,
        location::Location,
        task,
        update::Update,
        user::{Data, token::{request, response, Token}}},
    result,
};

#[derive(Debug)]
pub struct User {
    token: Token,
    pub http_client: reqwest::Client,
    pub data: Data,
    pub location: Location,
}

impl User {
    pub async fn from_refresh_token(token: String, location: Location, proxy: Option<reqwest::Proxy>) -> result::Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(header::ACCEPT_LANGUAGE, HeaderValue::from_str("en-US")?);

        let token = reqwest::Client::new()
            .post(google::identity_platform::ENDPOINT)
            .json(&request::Body {
                grant_type: "refresh_token",
                token: &token,
            })
            .send()
            .await?
            .json::<response::Body>()
            .await?
            .access_token;

        headers.insert(header::AUTHORIZATION, header::HeaderValue::try_from(&token)?);

        let client_builder = reqwest::Client::builder()
            .default_headers(headers);
        let http_client = match proxy {
            None => client_builder.build()?,
            Some(proxy) => client_builder
                .proxy(proxy)
                .build()?,
        };
        let data = Data::new(&http_client).await?;
        let user = Self {
            token,
            http_client,
            data,
            location,
        };

        user.update_location().await?;

        Ok(user)
    }

    pub async fn change_location(&mut self, location: &Location) -> reqwest::Result<()> {
        if &self.location != location {
            self.data = self.update_location().await?;
            self.location = location.clone();
        }

        Ok(())
    }

    pub async fn reserve(&self, task_id: Number) -> reqwest::Result<task::reserve::response::Body> {
        task::reserve::reserve(
            &self.http_client,
            &task::reserve::request::Body(vec![task_id]),
        ).await
    }

    pub async fn sync(&self) -> reqwest::Result<task::sync::response::Body> {
        Ok(
            task::sync::sync(
                &self.http_client,
                &task::sync::request::Body {
                    context_types: vec![context::Type::Empty],
                    deleted_reservation_ids: Vec::new(),
                    local_state: Vec::new(),
                    location: self.location.clone(),
                },
            ).await?
        )
    }

    async fn update_location(&self) -> reqwest::Result<Data> {
        self.location.clone().update(&self.http_client).await
    }
}
