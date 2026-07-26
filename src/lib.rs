modwire::expose! {
	pub audio_target
	pub choice
	pub completion
	pub configuration
	pub content
	pub image_target
	pub inference
	pub inference_request
	pub media
	pub message
	pub part
	pub request
	pub response_format
	pub role
	pub token_count
	pub usage
	pub verbosity
}

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
	#[error("{}", 0)]
	Reqwest(#[from] reqwest::Error),
	#[error("{}", 0)]
	SerdeJson(#[from] serde_json::Error),
	#[error("{}", 0)]
	Url(#[from] url::ParseError),
	#[error("{}", 0)]
	Parse(Box<dyn std::error::Error + Send + Sync + 'static>),
	#[error("{}", 0)]
	RequestFailed(String),
	#[error("completion empty")]
	CompletionEmpty,
	#[error("unsupported response content type")]
	Unsupported
}