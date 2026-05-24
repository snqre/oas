use super::*;

#[derive(Debug)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Completion {
	#[serde(rename = "id")]
	key: String,
	created: u64,
	model: String,
	#[serde(rename = "choices")]
	output: Vec<Choice>,
	#[serde(skip_serializing_if = "Option::is_none")]
	usage: Option<Usage>
}

#[bon::bon]
impl Completion {
	#[builder]
	pub async fn from_oas_provider<T>(
		client: Option<&reqwest::Client>,
		url: &url::Url,
		api_key: Option<&str>,
		request: &Request<T>
	) -> Result<Self> 
	where
		T: serde::Serialize {
		let url: &str = url.as_str();
		let client: &reqwest::Client = if let Some(client) = client {
			client
		} else {
			&reqwest::Client::default()
		};
		let content: serde_json::Value = serde_json::to_value(request)?;
		let mut request: reqwest::RequestBuilder = client.post(url).json(&content);
		if let Some(api_key) = api_key {
			request = request.bearer_auth(api_key);
		}
		let out: reqwest::Response = request.send().await?;
		let out: Self = out.json().await?;
		Ok(out)
	}
}

impl Completion {
	pub fn key(&self) -> &str {
		&self.key
	}
	
	pub fn created(&self) -> u64 {
		self.created
	}
	
	pub fn model(&self) -> &str {
		&self.model
	}
	
	pub fn output(&self) -> &[Choice] {
		&self.output
	}
	
	pub fn usage(&self) -> Option<&Usage> {
		if let Some(usage) = &self.usage {
			Some(usage)
		} else {
			None
		}
	}
}