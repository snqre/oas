use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Content {
	Text(String),
	Part(Vec<Part>)
}

impl From<String> for Content {
	fn from(value: String) -> Self {
    	Self::Text(value)
	}
}

impl From<Vec<Part>> for Content {
	fn from(value: Vec<Part>) -> Self {
		Self::Part(value)
	}
}