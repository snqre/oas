#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct ImageTarget {
	url: String
}

impl ImageTarget {
	pub fn from_url(url: url::Url) -> Self {
		let url: String = url.to_string();
		Self {
			url
		}
	}
}
