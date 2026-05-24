use super::*;

#[derive(Debug)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Choice {
	#[serde(rename = "index")]
	pub key: usize,
	pub message: Message,
	#[serde(rename = "finish_reason")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completion_cause: Option<String>
}
