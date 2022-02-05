use super::{currency::Currency, number::Number};

pub mod amount;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Price {
    pub value: Number,
    pub currency: Currency,
}
