//! OAuth validation request

// Imports
use crate::{oauth_url, OAuthRequest};

/// OAuth validation request
///
/// This request uses the `/oauth2/validate`
/// path to attempt to validate and receive
/// more information about an `oauth` token.
///
/// This request takes no arguments, as the
/// oauth token is passed on the html header,
/// as authorization.
///
/// # Examples
/// ```
/// # use twitch_helix::request::oauth::validate::Request;
/// # use twitch_helix::OAuthRequest;
/// let mut request = Request;
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("id.twitch.tv"));
/// assert_eq!(url.path(), "/oauth2/validate");
/// assert_eq!(url.query(), None);
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request;

/// OAuth validation response
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Response {
	/// Client id
	pub client_id: String,

	/// Login username
	pub login: String,

	/// Scopes
	pub scopes: Vec<String>,

	/// User id
	pub user_id: String,

	/// Expiration
	pub expires_in: Option<u64>,
}

impl OAuthRequest for Request {
	type Response = Response;

	fn url(&self) -> url::Url {
		oauth_url!(oauth2 / validate)
	}
}
