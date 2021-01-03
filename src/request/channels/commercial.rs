//! Channel start commercial request

// Imports
use crate::{helix_url, HelixRequest, HttpMethod};

/// Channel start commercial request
///
/// This request uses the `/channels/commercial` path
/// to start a commercial on a channel
///
/// # Examples
/// ```
/// # use twitch_helix::request::channel::commercial::{Request, Length};
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new("my-channel-id", Length::Seconds30);
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/channels/commercial");
/// assert_eq!(url.query(), Some("broadcaster_id=my-channel-id&length=30"));
/// ```
///
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request {
	/// Broadcaster ID
	pub broadcaster_id: String,

	/// Commercial length
	pub length: Length,
}

/// A commercial length
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Length {
	/// 30 seconds
	Seconds30,

	/// 60 seconds
	Seconds60,

	/// 90 seconds
	Seconds90,

	/// 120 seconds
	Seconds120,

	/// 150 seconds
	Seconds150,

	/// 180 seconds
	Seconds180,
}

impl Length {
	/// Returns the number of seconds of this length
	#[must_use]
	pub const fn secs(self) -> usize {
		match self {
			Self::Seconds30 => 30,
			Self::Seconds60 => 60,
			Self::Seconds90 => 90,
			Self::Seconds120 => 120,
			Self::Seconds150 => 150,
			Self::Seconds180 => 180,
		}
	}
}

impl Request {
	/// Creates a new request
	#[must_use]
	pub fn new(broadcaster_id: impl Into<String>, length: Length) -> Self {
		Self {
			broadcaster_id: broadcaster_id.into(),
			length,
		}
	}
}

impl HelixRequest for Request {
	type Response = [Response; 1];

	fn url(&self) -> url::Url {
		let mut url = helix_url!(channels / commercial);
		{
			let mut query_pairs = url.query_pairs_mut();
			query_pairs.append_pair("broadcaster_id", &self.broadcaster_id);
			query_pairs.append_pair("length", &self.length.secs().to_string());
		}
		url
	}

	fn http_method(&self) -> HttpMethod {
		HttpMethod::Post
	}
}

/// The response from the server
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
	/// Length of the commercial triggered
	pub length: usize,

	/// Seconds until the next commercial can be
	/// served on this channel
	pub retry_after: usize,
}
