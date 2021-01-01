//! Helix request trait

// Modules
pub mod channel;
pub mod games;
pub mod oauth;
pub mod search;

/// An http method
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum HttpMethod {
	/// Get
	Get,

	/// Post
	Post,
}

/// A Helix request
///
/// Every helix request has an associated response type. This type is
/// not returned directly from responses, as errors may occur.
/// A [`HelixResponse`](crate::HelixResponse) is returned, that includes
/// this response type inside it's `Ok` variant, in the `data` field.
///
/// A helix request must simply return it's url and what http method to
/// use to make the request.
pub trait HelixRequest {
	/// Response type
	type Response: for<'de> serde::Deserialize<'de>;

	/// Returns this request's url
	fn url(&self) -> url::Url;

	/// Returns the request's http method
	fn http_method(&self) -> HttpMethod;
}

/// An OAuth request
///
/// Every helix request has an associated response type. This type is
/// not returned directly from responses, as errors may occur.
/// A [`OAuthResponse`](crate::OAuthResponse) is returned, that includes
/// response type as it's `Ok` variant.
///
/// An oauth request must simply return it's url, as all requests use
/// the `Get` http method.
pub trait OAuthRequest {
	/// Response type
	type Response: for<'de> serde::Deserialize<'de>;

	/// Returns this request's url
	fn url(&self) -> url::Url;
}
