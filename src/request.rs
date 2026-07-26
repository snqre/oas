use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(bon::Builder)]
pub struct Request<T = ()> {
	#[builder(into)]
	pub model: String,
	#[serde(rename = "max_completion_tokens")]
	#[builder(into)]
	pub max_completion: TokenCount,
	#[serde(rename = "messages")]
	#[builder(into)]
	pub context: Vec<Message>,
	#[builder(default = 1.0)]
	pub frequency_penalty: f32,
	#[builder(default = 1.0)]
	pub temperature: f32,
	#[builder(default = 1.0)]
	pub top_p: f32,
	#[builder(default = 1.0)]
	pub presence_penalty: f32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verbosity: Option<Verbosity>,
	pub response_format: ResponseFormat<T>
}