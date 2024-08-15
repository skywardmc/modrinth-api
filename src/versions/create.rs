//! Models for Modrinth's `/version` endpoint.
//!
//! See [the docs](https://docs.modrinth.com/#tag/versions/operation/createVersion)
use crate::{RequestedVersionStatus, VersionDependency, VersionStatus, VersionType};

use serde::{Deserialize, Serialize};

/// Loader used by the project
#[derive(
	Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[non_exhaustive]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum Loader {
	#[default]
	Fabric,
	Forge,
	Quilt,
	Neoforge,
}

/// POST `/version`
///
/// `data` multipart to POST to Modrinth.
/// See [the docs](https://docs.modrinth.com/#tag/versions/operation/createVersion)
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreatableVersionData {
	/// The name of this version
	pub name: String,
	/// The version number. Ideally will follow semantic versioning
	pub version_number: String,
	/// The changelog for this version
	pub changelog: Option<String>,
	/// A list of specific versions of projects that this version depends on
	pub dependencies: Vec<VersionDependency>,
	/// A list of versions of Minecraft that this version supports
	pub game_versions: Vec<String>,
	/// The release channel for this version
	pub version_type: VersionType,
	/// The mod loaders that this version supports
	pub loaders: Vec<Loader>,
	/// Whether the version is featured or not
	pub featured: bool,
	pub status: Option<VersionStatus>,
	pub requested_status: Option<RequestedVersionStatus>,
	/// The ID of the project this version is for
	pub project_id: String,
	/// An array of the multipart field names of each file that goes with this version
	pub file_parts: Vec<String>,
	/// The multipart field name of the primary file
	pub primary_file: String,
}
