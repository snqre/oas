use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Configuration<T> {
	#[serde(skip)]
	phantom_data: std::marker::PhantomData<T>,
	name: String,
	schema: serde_json::Value
}

impl<T> Configuration<T>
where
	T: schemars::JsonSchema {
	pub fn from_schema() -> Result<Self> {
		Ok(Self {
			phantom_data: std::marker::PhantomData,
			name: String::from("structured_output"),
			schema: serde_json::to_value(schemars::schema_for!(T))?
		})
	}
}