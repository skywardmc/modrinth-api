//! Structured bindings to Modrinth's API
use serde::{Deserialize, Serialize};

pub mod version_files;
pub mod versions;
pub use version_files::*;
pub use versions::*;

#[derive(
	Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum HashType {
	Sha1,
	#[default]
	Sha512,
}

/// Type of project dependency
#[derive(
	Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum DependencyType {
	Embedded,
	Incompatible,
	Optional,
	#[default]
	Required,
}

/// Type of project version
#[derive(
	Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum VersionType {
	Alpha,
	Beta,
	#[default]
	Release,
}
