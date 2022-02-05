use super::{estimated_duration::EstimatedDuration, image::Thumbnail};

pub mod survey;
pub mod relevance;
pub mod group;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormLocalization {
    pub category_label: String,
    pub summary: String,
    pub formatted_summary: Option<String>,
    pub survey: survey::Survey,
    pub locale: String,
    #[serde(rename = "taskThumbnailImageURL")]
    #[serde(skip)]
    pub thumbnail_image_url: String,
    #[serde(rename = "taskThumbnailImage")]
    pub thumbnail_image: Thumbnail,
    pub tier: String,
    pub title: String,
    #[serde(rename = "estimatedTaskDuration")]
    pub estimated_duration: EstimatedDuration,
    pub tags: Option<Vec<String>>,
}
