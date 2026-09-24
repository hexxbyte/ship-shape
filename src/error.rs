use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};

/// Everything that can go wrong while checking for, downloading, or verifying an update.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateError {
	/// A version string could not be parsed as semver.
	InvalidVersion(String),
	/// The server answered with a non-success HTTP status code.
	Http(u16),
	/// The connection failed or timed out.
	Network(String),
	/// The GitHub API returned something unexpected.
	InvalidResponse(String),
	/// The release has no asset matching this platform and install kind.
	NoDownload(String),
	/// Reading or writing a local file failed.
	Io(String),
	/// The minisign public key or signature is invalid, or the download does not match it.
	Verification(String),
	/// The in-progress download was cancelled by the caller via the `cancelled` flag
	/// passed to [`download_update_file`](crate::download_update_file).
	Cancelled,
}

impl Display for UpdateError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidVersion(msg) => write!(f, "Invalid version: {msg}"),
			Self::Http(code) => write!(f, "HTTP error: {code}"),
			Self::Network(msg) => write!(f, "Network error: {msg}"),
			Self::InvalidResponse(msg) => write!(f, "Invalid response: {msg}"),
			Self::NoDownload(msg) => write!(f, "No download: {msg}"),
			Self::Io(msg) => write!(f, "I/O error: {msg}"),
			Self::Verification(msg) => write!(f, "Verification error: {msg}"),
			Self::Cancelled => write!(f, "Download cancelled"),
		}
	}
}

impl Error for UpdateError {}

impl From<ureq::Error> for UpdateError {
	fn from(err: ureq::Error) -> Self {
		match err {
			ureq::Error::StatusCode(code) => Self::Http(code),
			_ => Self::Network(err.to_string()),
		}
	}
}
