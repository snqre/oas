use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(untagged)]
pub enum Content {
	Text(String),
	Part(Vec<Part>)
}