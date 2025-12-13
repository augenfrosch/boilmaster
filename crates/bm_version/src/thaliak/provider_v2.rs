use anyhow::Result;
use nonempty::NonEmpty;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Patch {
	pub version_string: String,
	pub remote_url: String,
	pub size: u64,
	// old TODO: "hashes (needs fixes @ thaliak)", hashes are part of v2 but using them is not too straightforward
}

#[derive(Debug, Deserialize)]
struct RepositoryPatchesResponse {
	patches: Vec<Patch>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	endpoint: String,
}

pub struct Provider {
	endpoint: String,
	client: reqwest::Client,
}

impl Provider {
	pub fn new(config: Config) -> Self {
		Self {
			endpoint: config.endpoint,
			client: reqwest::Client::new(),
		}
	}

	#[tracing::instrument(level = "debug", skip(self))]
	pub async fn patch_list(&self, repository: String) -> Result<NonEmpty<Patch>> {
		let response = self
			.client
			.get(&format!(
				"{endpoint}/repositories/{repository}/patches",
				endpoint = self.endpoint
			))
			.send()
			.await?
			.json::<RepositoryPatchesResponse>()
			.await?;

		// TODO: check behaviour when endpoint is unreachabke. REST API should ensure we have a valid response here without any error

		let patches = response.patches;

		// TODO: determine if the single-patch history workaround is still needed (probably?)

		NonEmpty::from_vec(patches)
			.ok_or_else(|| anyhow::anyhow!("response's deserialized patch list is empty"))
	}
}
