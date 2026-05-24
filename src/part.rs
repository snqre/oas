use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Part {
	Text {
		#[serde(rename = "text")]
		content: String
	},
	#[serde(rename = "image_url")]
	Image {
		#[serde(rename = "image_url")]
		content: ImageTarget
	},
	#[serde(rename = "input_audio")]
	Audio {
		#[serde(rename = "input_audio")]
		content: AudioTarget
	}
}

impl Part {
	pub fn from_text(content: String) -> Self {
		Self::Text {
			content
		}
	}

	pub fn from_image_url(url: url::Url) -> Self {
		Self::Image {
			content: ImageTarget::from_url(url)
		}
	}
	
	pub fn from_image_bytes(bytes: &[u8]) -> Result<Self> {
		let url: Media = bytes.into();
		let url: String = url.uri();
		let url: url::Url = url.parse()?;
		Ok(Self::from_image_url(url))
	}
	
	pub fn from_audio_bytes(bytes: &[u8]) -> Self {
		let media: Media = bytes.into();
		Self::Audio {
			content: AudioTarget::from_media(media)
		}
	}
}
