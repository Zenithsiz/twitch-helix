//! OAuth validation request

// Imports
use crate::{oauth_url, OAuthRequest};

/// OAuth validation request
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request;

/// Channel search output
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Output {
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
	type Output = Output;

	fn url(&self) -> url::Url {
		oauth_url!(oauth2 / validate)
	}
}
