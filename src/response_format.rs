use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat<T = ()> {
	Text,
	JsonObject,
	#[serde(rename = "json_schema")]
	#[from]
	JsonSchema {
		#[serde(rename = "json_schema")]
		configuration: Configuration<T>
	}
}