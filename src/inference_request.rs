use super::*;

#[derive(bon::Builder)]
pub struct InferenceRequest<'a, T> {
	#[builder(skip)]
	phantom_data: std::marker::PhantomData<T>,
	client: Option<&'a reqwest::Client>,
    url: &'a url::Url,
    api_key: Option<&'a str>,
    #[builder(into)]
    model: String,
    #[builder(default = 2000)]
    #[builder(into)]
    max_completion: TokenCount,
    #[builder(default = 1.0)]
    frequency_penalty: f32,
    #[builder(default = 1.0)]
    temperature: f32,
    #[builder(default = 1.0)]
    top_p: f32,
    #[builder(default = 1.0)]
    presence_penalty: f32,
    verbosity: Option<Verbosity>,
    #[builder(into)]
	context: Vec<Message>,
}

impl<'a, T> InferenceRequest<'a, T> 
where
	T: serde::Serialize,
	T: serde::de::DeserializeOwned,
	T: schemars::JsonSchema {
	pub async fn infer(self) -> Result<T> {
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
		let content: Request<T> = Request::builder()
    		.model(self.model)
      		.max_completion(self.max_completion)
        	.frequency_penalty(self.frequency_penalty)
         	.temperature(self.temperature)
	        .top_p(self.top_p)
	        .presence_penalty(self.presence_penalty)
	        .maybe_verbosity(self.verbosity)
	        .context(self.context)
			.response_format(ResponseFormat::from_json_schema(Configuration::from_schema()?))
    		.build();
		let content: serde_json::Value = serde_json::to_value(&content)?;
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

impl<'a, T> InferenceRequest<'a, T> 
where
	T: std::str::FromStr,
	T::Err: std::error::Error,
	T::Err: Send,
	T::Err: Sync,
	T::Err: 'static {
	pub async fn infer_unstructured(self) -> Result<T> {
		let url: url::Url = self.url.join("chat/completions").unwrap();
		let url: String = url.to_string();
		let mut request: reqwest::RequestBuilder = if let Some(client) = self.client {
			client.post(&url)
		} else {
			reqwest::Client::default().post(&url)
		};
		if let Some(api_key) = self.api_key {
			request = request.bearer_auth(api_key);
		}
		let content: Request = Request::builder()
    		.model(self.model)
      		.max_completion(self.max_completion)
        	.frequency_penalty(self.frequency_penalty)
         	.temperature(self.temperature)
	        .top_p(self.top_p)
	        .presence_penalty(self.presence_penalty)
	        .maybe_verbosity(self.verbosity)
	        .context(self.context)
			.response_format(ResponseFormat::Text)
    		.build();
		let content: serde_json::Value = serde_json::to_value(&content)?;
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
				let out: T = out.parse().map_err(|error| Error::Parse(Box::new(error)))?;
				Ok(out)
			},
			content::Content::Part(_) => Err(Error::Unsupported)
		}
	}
}