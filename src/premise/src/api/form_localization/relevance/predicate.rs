use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
	pub node_type: NodeType,
	#[serde(rename = "ref")]
	pub referral: Option<Referral>,
	pub selected_value: Option<Box<SelectedValue>>,
	#[serde(flatten)]
	pub operation: Option<Box<Operation>>,
}

impl Predicate {
	pub fn evaluate(&self, selected: &HashMap<String, String>) -> bool {
		const NODE_TYPE_ERROR: &str = "Invalid node type.";

		match &self.selected_value {
			None => {
				match &self.operation {
					None => false,
					Some(operation) => {
						match operation.as_ref() {
							Operation::Binary { left, right } => {
								match self.node_type {
									NodeType::Or => left.evaluate(&selected) || right.evaluate(&selected),
									_ => panic!("{}", NODE_TYPE_ERROR),
								}
							}
						}
					}
				}
			}
			Some(selected_value) => {
				match (&self.node_type, &selected_value.value) {
					(NodeType::Not, Value::Boolean(value)) => !value,
					(NodeType::IsSelected, Value::Text(text)) => {
						match &self.referral {
							None => return false,
							Some(referral) => selected.get(&referral.input_name) == Some(text),
						}
					}
					_ => panic!("{}", NODE_TYPE_ERROR),
				}
			}
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
	Not,
	Or,
	Text,
	Boolean,
	IsSelected,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Referral {
	pub group_name: String,
	pub input_name: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedValue {
	pub node_type: NodeType,
	pub value: Value,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(untagged)]
pub enum Value {
	Boolean(bool),
	Text(String),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(untagged)]
pub enum Operation {
	Binary { left: Predicate, right: Predicate }
}
