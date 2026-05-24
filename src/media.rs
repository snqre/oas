use base64::Engine as _;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Media {
	mime: String,
	extension: String,
	base64: String
}

impl Media {
	pub fn uri(&self) -> String {
		format!("data:{};base64,{}", self.mime, self.base64)
	}

	pub fn mime(&self) -> &str {
		&self.mime
	}

	pub fn extension(&self) -> &str {
		&self.extension
	}

	pub fn base64(&self) -> &str {
		&self.base64
	}
}

impl From<&[u8]> for Media {
	fn from(value: &[u8]) -> Self {
		let base64: String = base64::engine::general_purpose::STANDARD.encode(value);
		if let Some(r#type) = infer::get(value) {
			Self {
				mime: r#type.mime_type().to_owned(),
				extension: r#type.extension().to_owned(),
				base64
			}
		} else {
			Self {
				mime: String::from("application/octet-stream"),
				extension: String::from("bin"),
				base64
			}
		}
	}
}
