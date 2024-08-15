//! Models for Modrinth's `/version_files` endpoint
//!
//! See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionsFromHashes)
use crate::{GetVersionFromHashResponse, HashType};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// POST `/version_files`
///
/// See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionsFromHashes)
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub struct GetVersionsFromHashesBody {
	pub hashes: Vec<String>,
	pub algorithm: HashType,
}

/// POST `/version_files`
///
/// See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionsFromHashes)
pub type GetVersionsFromHashesResponse = HashMap<String, GetVersionFromHashResponse>;

#[cfg(test)]
mod tests {
	use super::GetVersionsFromHashesResponse;

	use std::fs::OpenOptions;

	#[test]
	fn deserialize_response() {
		let file = OpenOptions::new()
			.read(true)
			.open(concat!(
				env!("CARGO_MANIFEST_DIR"),
				"/tests/fixtures/versions_from_hashes.json"
			))
			.unwrap();

		let parsed: Result<GetVersionsFromHashesResponse, _> = serde_json::from_reader(file);
		assert!(parsed.is_ok());
	}
}
