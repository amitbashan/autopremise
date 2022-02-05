#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfiguration {
    #[serde(rename = "networkId")]
    pub id: String,
    #[serde(rename = "networkName")]
    pub name: String,
    pub should_show_payment: bool,
    #[serde(flatten)]
    pub attributes: Attributes,
    #[serde(skip)]
    #[serde(rename = "splashscreens")]
    splash_screens: Vec<()>,
    pub groups: Groups,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub can_reserve_tasks: bool,
    pub require_passcode: bool,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(transparent)]
pub struct Groups(pub Vec<Group>);

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
}
