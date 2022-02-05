use serde_with::{DisplayFromStr, serde_as};
use url::Url;

use crate::{api::time::Time, make_endpoint};

pub mod name;
pub mod network_configuration;
pub mod token;

const ENDPOINT: &str = make_endpoint!(v2/users/authenticate);

#[serde_as]
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub email: String,
    #[serde(rename = "created")]
    pub creation_date: Time,
    #[serde(flatten)]
    pub name: name::Name,
    pub year_of_birth: usize,
    pub city: String,
    pub country: String,
    #[serde(rename = "profileImageURL")]
    pub profile_image: Option<Url>,
    #[serde_as(as = "DisplayFromStr")]
    pub id: serde_json::Number,
    pub external_id: uuid::Uuid,
    #[serde(rename(deserialize = "networkConfigs"))]
    pub network_configurations: Vec<network_configuration::NetworkConfiguration>,
    pub should_show_payments: bool,
    #[serde(flatten)]
    pub attributes: network_configuration::Attributes,
    pub use_custom_camera: bool,
    pub effectively_onboarded: bool,
    pub onboarded: bool,
    pub location_monitoring_enabled: bool,
    pub passive_monitoring_frequency_seconds: usize,
    pub passive_monitoring_location_accuracy_meters: usize,
    pub hide_personal_id_info: bool,
    pub auto_focus_enabled: bool,
}

impl Data {
    pub async fn new(client: &reqwest::Client) -> reqwest::Result<Self> {
        client.post(ENDPOINT)
            .header(reqwest::header::CONTENT_LENGTH, 0)
            .send()
            .await?
            .json()
            .await
    }
}
