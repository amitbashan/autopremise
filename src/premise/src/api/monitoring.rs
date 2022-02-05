use serde_json::Number;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Monitoring {
    #[serde(rename = "monitoringFrequencySeconds")]
    pub frequency: Number,
    #[serde(rename = "taskMonitoringRequired")]
    pub required: bool,
}
