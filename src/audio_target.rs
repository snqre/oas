use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct AudioTarget {
	data: String,
	format: String
}

impl AudioTarget {
	pub fn from_media(media: Media) -> Self {
		let data: String = media.base64().to_owned();
		let format: String = media.extension().to_owned();
		Self::from_data_and_format(data, format)
	}
	
	pub fn from_data_and_format(data: String, format: String) -> Self {
		Self {
			data,
			format
		}
	}
}