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
	pub frequency_penalty: f32,
	pub temperature: f32,
	pub top_p: f32,
	pub presence_penalty: f32,
	pub verbosity: Verbosity,
	pub response_format: ResponseFormat<T>
}