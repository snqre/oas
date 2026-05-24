use super::*;

#[derive(bon::Builder)]
pub struct InferenceRequest<'a, T> {
	client: Option<&'a reqwest::Client>,
    url: &'a url::Url,
    api_key: Option<&'a str>,
    request: &'a Request<T>
}

impl<'a, T> InferenceRequest<'a, T> 
where
	T: serde::Serialize,
	T: serde::de::DeserializeOwned {
	pub async fn send(self) -> Result<T> {
		let url: url::Url = self.url.join("chat/completions")?;
		let url: String = url.to_string();
		let mut request: reqwest::RequestBuilder = if let Some(client) = self.client {
			client.post(&url)
		} else {
			reqwest::Client::default().post(&url)
		};
		if let Some(api_key) = self.api_key {
			request = request.bearer_auth(api_key);
		}
		let content: serde_json::Value = serde_json::to_value(&self.request)?;
		let response: reqwest::Response = request.json(&content).send().await?;
		if !response.status().is_success() {
			let error: String = response.text().await?;
			let error: Error = Error::RequestFailed(error);
			return Err(error)
		}
		let completion: serde_json::Value = response.json().await?;
		let completion: completion::Completion = serde_json::from_value(completion)?;
		let content: &choice::Choice = completion.output().first().ok_or(Error::CompletionEmpty)?;
		let content: &content::Content = &content.message.content;
		match content {
			content::Content::Text(s) => {
				let out: String = s.to_owned();
				let out: T = serde_json::from_str(&out)?;
				Ok(out)
			},
			content::Content::Part(_) => Err(Error::Unsupported)
		}
	}
}