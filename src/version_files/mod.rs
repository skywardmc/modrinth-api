//! Models for Modrinth's Version Files endpoints
//!
//! See [the docs](https://docs.modrinth.com/#tag/version-files)
use serde::{Deserialize, Serialize};

pub mod from_hash;
pub mod from_hashes;
pub use from_hash::*;
pub use from_hashes::*;

/// Version of project another depends on
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VersionDependency {
	pub version_id: Option<String>,
	pub project_id: Option<String>,
	pub file_name: Option<String>,
	pub dependency_type: crate::DependencyType,
}

/// Status of new project version
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum VersionStatus {
	Archived,
	Draft,
	#[default]
	Listed,
	Scheduled,
	Unknown,
	Unlisted,
}

/// Requested status of new project version
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum RequestedVersionStatus {
	Archived,
	Draft,
	#[default]
	Listed,
	Unlisted,
}
