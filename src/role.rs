#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(rename_all = "snake_case")]
pub enum Role {
	#[default]
	System,
	User,
	Assistant
}
