```rust
use oas::Inference as _;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(schemars::JsonSchema)]
pub struct Score {
	lhs_team: u8,
	rhs_team: u8
}

impl Score {
	pub async fn from_inference() -> Result<Self> {
		Self::from_oas_provider()
		    .api_key("...")
		    .url(&url::Url::parse("")?)
			.model("grok")
			.max_completion(2000)
			.temperature(2.0)
			.top_p(1.0)
			.context([
				Message::builder().role(Role::System).content(String::from("simulate a football match")).build(),
				Message::builder().role(Role::System).content(Vec::from([
					Part::from_text(String::from("make no mistakes")),
				]))
				.build()
			])
			.build()
			.infer().await
	}
}
```