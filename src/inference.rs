use super::*;

pub trait Inference<'a>
where
	Self: Sized {
	fn from_oas_provider() -> InferenceRequestBuilder<'a, Self>;
}

impl<'a, T> Inference<'a> for T {
	fn from_oas_provider() -> InferenceRequestBuilder<'a, T> {
		InferenceRequest::builder()
	}
}
