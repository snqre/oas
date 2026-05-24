use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(bon::Builder)]
pub struct Message {
	pub role: Role,
	#[builder(into)]
	pub content: Content
}