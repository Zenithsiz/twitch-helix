//! Helix request trait

// Imports
use reqwest as req;

/// A Helix request
pub trait HelixRequest {
	/// Output type
	type Output: for<'de> serde::Deserialize<'de>;

	/// Returns this request's url
	fn url(&self) -> url::Url;

	/// Returns the request's http method
	fn http_method(&self) -> req::Method;
}

/// An OAuth request
pub trait OAuthRequest {
	/// Output type
	type Output: for<'de> serde::Deserialize<'de>;

	/// Returns this request's url
	fn url(&self) -> url::Url;
}
