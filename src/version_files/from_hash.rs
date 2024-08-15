//! Models for Modrinth's `/version_file/{hash}` endpoint
//!
//! See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionFromHash)
use crate::{HashType, RequestedVersionStatus, VersionDependency, VersionStatus, VersionType};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// File included in a version of a project
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VersionFile {
	/// Hashes of this file
	pub hashes: HashMap<HashType, String>,
	/// Download URL for this file
	pub url: String,
	/// Name of this file
	pub filename: String,
	/// Whether this is the primary file
	pub primary: bool,
	/// Size of the file in bytes
	pub size: u64,
	/// Type of the file
	pub file_type: Option<String>,
}

/// GET `/version_file/{hash}`
///
/// See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionFromHash)
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetVersionFromHashResponse {
	/// Name of this version
	pub name: String,
	/// Ideally will follow semantic versioning
	pub version_number: String,
	/// Changelog for this version
	pub changelog: Option<String>,
	/// List of specific versions of projects that this version depends on
	pub dependencies: Vec<VersionDependency>,
	/// List of versions of Minecraft that this version supports
	pub game_versions: Vec<String>,
	/// Release channel for this version
	pub version_type: VersionType,
	/// Mod loaders that this version supports
	pub loaders: Vec<String>,
	/// Whether the version is featured or not
	pub featured: bool,
	pub status: Option<VersionStatus>,
	pub requested_status: Option<RequestedVersionStatus>,
	/// ID of this version
	pub id: String,
	/// ID of the project this version is for
	pub project_id: String,
	/// ID of the author of this version
	pub author_id: String,
	pub date_published: String,
	/// Number of downloads this file has had
	pub downloads: u64,
	/// URL to this versions's changelog
	pub changelog_url: Option<String>,
	/// An array of the files this version has
	pub files: Vec<VersionFile>,
}

/// GET `/version_file/{hash}`
///
/// See [the docs](https://docs.modrinth.com/#tag/version-files/operation/versionFromHash)
#[derive(
	Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub struct GetVersionFromHashParams {
	/// Algorithm of the hash
	pub algorithm: HashType,
	/// Whether to return multiple results when looking for this hash
	pub multiple: bool,
}

#[cfg(test)]
mod tests {
	use super::GetVersionFromHashResponse;

	use std::fs::OpenOptions;

	#[test]
	fn deserialize_response() {
		let file = OpenOptions::new()
			.read(true)
			.open(concat!(
				env!("CARGO_MANIFEST_DIR"),
				"/tests/fixtures/version_from_hash.json"
			))
			.unwrap();

		let parsed: Result<GetVersionFromHashResponse, _> = serde_json::from_reader(file);
		assert!(parsed.is_ok());
	}
}
