//! Channel info request

// Imports
use crate::{helix_url, HelixRequest};
use reqwest as req;

/// Channel info request
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request {
	/// Broadcaster ID
	broadcaster_id: String,
}

impl Request {
	/// Creates a new request
	#[must_use]
	pub const fn new(broadcaster_id: String) -> Self {
		Self { broadcaster_id }
	}

	/// Returns the specific channel found by this request's
	/// response
	///
	/// It attempts to exact match any channel in the output
	/// with the request, if found, it is returned, else
	/// `None` is returned.
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

	/// Returns the specific channel found by this request's
	/// response by ref
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
	type Output = Vec<Channel>;

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
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
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
