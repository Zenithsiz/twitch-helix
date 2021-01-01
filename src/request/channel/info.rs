//! Channel info request

// Imports
use crate::{helix_url, HelixRequest};
use reqwest as req;

/// Channel info request
///
/// This request uses the `/channels` path
/// to get information about a channel
/// given their broadcaster id.
///
/// # Examples
/// ```
/// # use twitch_helix::request::channel::info::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new("my-channel-id");
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/channels");
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

	/// Finds the exact channel requested given the response
	///
	/// Attempts to find an exact match in the `display_name`
	/// field of the channel, without considering case.
	#[must_use]
	pub fn channel(&self, channels: Vec<Channel>) -> Option<Channel> {
		// Check every channel in the response
		for channel in channels {
			if unicase::eq(&self.broadcaster_id, &channel.broadcast_id) {
				return Some(channel);
			}
		}

		// If we get here, no channel was found
		None
	}

	/// Finds the exact channel requested given the response by reference.
	///
	/// See [`Self::channel`] for more information.
	#[must_use]
	pub fn channel_ref<'a>(&self, channels: &'a [Channel]) -> Option<&'a Channel> {
		// Check every channel in the response
		for channel in channels {
			if unicase::eq(&self.broadcaster_id, &channel.broadcast_id) {
				return Some(channel);
			}
		}

		// If we get here, no channel was found
		None
	}
}

impl HelixRequest for Request {
	type Response = Vec<Channel>;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(channels);
		url.query_pairs_mut().append_pair("broadcaster_id", &self.broadcaster_id);
		url
	}

	fn http_method(&self) -> req::Method {
		req::Method::GET
	}
}

/// Each channel in the output data
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Channel {
	/// Channel streaming status
	pub status: String,

	/// Broadcaster id
	pub broadcast_id: String,

	/// Game id
	pub game_id: String,

	/// Broadcaster_language,
	pub broadcaster_language: String,

	/// Title
	pub title: String,

	/// Description
	pub description: String,
}
