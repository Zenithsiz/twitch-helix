//! Cheermotes request

// Imports
use crate::{helix_url, HelixRequest, HttpMethod};

/// Cheermotes request
///
/// This request uses the `/bits/cheermotes` path
/// to get a list of all available cheermotes.
///
/// # Examples
/// ```
/// # use twitch_helix::request::bits::cheermotes::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new("my-channel-id");
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/bits/cheermotes");
/// assert_eq!(url.query(), Some("broadcaster_id=my-channel-id"));
/// ```
///
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request {
	/// Broadcaster ID
	broadcaster_id: String,
}

impl Request {
	/// Creates a new request
	#[must_use]
	pub fn new(broadcaster_id: impl Into<String>) -> Self {
		Self {
			broadcaster_id: broadcaster_id.into(),
		}
	}
}

impl HelixRequest for Request {
	type Response = Vec<Cheermote>;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(bits / cheermotes);
		url.query_pairs_mut().append_pair("broadcaster_id", &self.broadcaster_id);
		url
	}

	fn http_method(&self) -> HttpMethod {
		HttpMethod::Get
	}
}

/// A cheermote
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cheermote {
	/// Prefix
	prefix: String,

	/// Tiers
	tiers: Vec<CheermoteTier>,

	/// Type
	#[serde(rename = "type")]
	cheermote_type: String,

	/// Order
	order: usize,
	///// Last updated
}

/// A cheermote tier
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CheermoteTier {}
