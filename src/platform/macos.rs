use std::{
	env,
	path::{Path, PathBuf},
	process::Command,
};

use patois::t;

use super::InstallOutcome;
use crate::{InstallKind, UpdateError, UpdaterConfig};

/// macOS has a single asset kind, a disk image, so `install_kind` is ignored.
pub const fn asset_name_parts(_install_kind: InstallKind) -> (&'static str, &'static str) {
	("", "dmg")
}

#[expect(clippy::unnecessary_wraps, reason = "must match the signature of the other platforms")]
pub fn download_dir(_config: &UpdaterConfig) -> Result<PathBuf, UpdateError> {
	Ok(env::temp_dir())
}

/// Mount the downloaded disk image and prompt the user to finish installing by hand.
///
/// There's no way to self-replace a running, Gatekeeper-checked `.app` bundle the way the
/// Windows install script does, so this stops at opening the mounted image; dragging the app
/// into Applications is left to the user.
pub fn install(config: &UpdaterConfig, path: &Path) -> Result<InstallOutcome, String> {
	// `open` mounts the disk image and shows it in a Finder window, the same as
	// double-clicking it.
	Command::new("open").arg(path).spawn().map_err(|e| format!("{}: {e}", t("Failed to open disk image")))?;
	Ok(InstallOutcome::ManualStep(
		t(
			"The update has been downloaded and its disk image opened. Quit %s and drag the new version into Applications to finish installing.",
		)
		.replace("%s", &config.app_display_name),
	))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn asset_is_always_a_dmg() {
		assert_eq!(asset_name_parts(InstallKind::Installer), ("", "dmg"));
		assert_eq!(asset_name_parts(InstallKind::Portable), ("", "dmg"));
	}
}
