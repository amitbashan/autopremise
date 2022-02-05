use serde_with::serde_as;

use crate::{
    api::{
        form_localization::group,
        location::upload::UploadLocation,
        number::Number,
        service_data::ServiceData,
        time::Time,
    },
};

#[serde_as]
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub user_id: Number,
    #[serde(flatten)]
    pub reservation_info: super::super::sync::response::reservation::Info,
    pub client_submitted_time: Time,
    pub output_group_results: Vec<group::output::OutputGroup>,
    pub upload_location: UploadLocation,
    pub client_submission_id: uuid::Uuid,
    pub service_data: Option<ServiceData>,
}
