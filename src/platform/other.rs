use std::{
	env,
	path::{Path, PathBuf},
};

use patois::t;

use super::InstallOutcome;
use crate::{InstallKind, UpdateError, UpdaterConfig};

/// Fallback for platforms without an install flow: reuse the Windows asset names and leave the
/// install to the user.
pub const fn asset_name_parts(install_kind: InstallKind) -> (&'static str, &'static str) {
	match install_kind {
		InstallKind::Installer => ("_setup", "exe"),
		InstallKind::Portable => ("", "zip"),
	}
}

#[expect(clippy::unnecessary_wraps, reason = "must match the signature of the other platforms")]
pub fn download_dir(_config: &UpdaterConfig) -> Result<PathBuf, UpdateError> {
	Ok(env::temp_dir())
}

#[expect(clippy::unnecessary_wraps, reason = "must match the signature of the other platforms")]
pub fn install(_config: &UpdaterConfig, path: &Path) -> Result<InstallOutcome, String> {
	Ok(InstallOutcome::ManualStep(
		t("Update downloaded to: %s\nPlease install it manually.").replace("%s", &path.display().to_string()),
	))
}
