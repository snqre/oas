use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Usage {
	#[serde(rename = "prompt_tokens")]
	pub prompt: TokenCount,
	#[serde(rename = "completion_tokens")]
	pub completion: TokenCount
}

impl Usage {
	pub fn prompt(&self) -> token_count::TokenCount {
		self.prompt
	}
	
	pub fn completion(&self) -> token_count::TokenCount {
		self.completion
	}
	
	pub fn total(&self) -> token_count::TokenCount {
		self.prompt + self.completion
	}
}