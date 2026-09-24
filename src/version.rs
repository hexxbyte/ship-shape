pub fn parse_semver(value: &str) -> Option<(u64, u64, u64)> {
	let trimmed = value.trim();
	if trimmed.is_empty() {
		return None;
	}
	let normalized = trimmed.trim_start_matches(['v', 'V']);
	let mut parts = normalized.split('.').map(|p| p.split_once('-').map_or(p, |(v, _)| v));
	let major = parts.next()?.parse().ok()?;
	let minor = parts.next().unwrap_or("0").parse().ok()?;
	let patch = parts.next().unwrap_or("0").parse().ok()?;
	Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn semver_accepts_prefixes_and_prerelease_suffix() {
		assert_eq!(parse_semver("v1.2.3"), Some((1, 2, 3)));
		assert_eq!(parse_semver("V4.5.6"), Some((4, 5, 6)));
		assert_eq!(parse_semver("1.2.3-beta.1"), Some((1, 2, 3)));
	}

	#[test]
	fn semver_defaults_missing_parts() {
		assert_eq!(parse_semver("1"), Some((1, 0, 0)));
		assert_eq!(parse_semver("1.2"), Some((1, 2, 0)));
	}

	#[test]
	fn semver_rejects_empty_or_invalid() {
		assert_eq!(parse_semver(""), None);
		assert_eq!(parse_semver("not-a-version"), None);
		assert_eq!(parse_semver("v"), None);
		assert_eq!(parse_semver(".2.3"), None);
	}

	#[test]
	fn semver_trims_whitespace() {
		assert_eq!(parse_semver("  v2.3.4  "), Some((2, 3, 4)));
	}

	#[test]
	fn semver_ignores_extra_segments() {
		assert_eq!(parse_semver("1.2.3.99"), Some((1, 2, 3)));
	}
}
