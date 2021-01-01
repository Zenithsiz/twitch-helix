//! Channel search request

// Imports
use crate::{helix_url, HelixRequest, HttpMethod};

/// Channel search request
///
/// This request uses the `/search/channels` path
/// to search channels by a query string.
///
/// Response is a list of `[Channel]s`.
///
/// # Examples
/// Simple request:
/// ```
/// # use twitch_helix::request::search::channel::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new("my-channel");
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/search/channels");
/// assert_eq!(url.query(), Some("query=my-channel"));
/// ```
///
/// Using every argument:
/// ```
/// # use twitch_helix::request::search::channel::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new("my-channel");
/// request.first     = Some(100);
/// request.after     = Some("my-cursor".to_string());
/// request.live_only = Some(true);
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/search/channels");
/// assert_eq!(url.query(), Some("query=my-channel&first=100&after=my-cursor&live_only=true"));
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Request {
	/// Search query
	pub query: String,

	/// Maximum number of objects to return
	pub first: Option<usize>,

	/// Cursor for forward pagination
	pub after: Option<String>,

	/// Filter results for live streams only.
	pub live_only: Option<bool>,
}

impl Request {
	/// Creates a new channel search request given
	/// the query to search for
	pub fn new(query: impl Into<String>) -> Self {
		Self {
			query: query.into(),
			first: None,
			after: None,
			live_only: None,
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
			if unicase::eq(&self.query, &channel.display_name) {
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
			if unicase::eq(&self.query, &channel.display_name) {
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
		// Append all our arguments if they exist
		let mut url = helix_url!(search / channels);
		let mut query_pairs = url.query_pairs_mut();
		query_pairs.append_pair("query", &self.query);
		if let Some(first) = &self.first {
			query_pairs.append_pair("first", &first.to_string());
		}
		if let Some(after) = &self.after {
			query_pairs.append_pair("after", after);
		}
		if let Some(live_only) = &self.live_only {
			query_pairs.append_pair("live_only", &live_only.to_string());
		}

		// Drop the query pairs and return the url
		std::mem::drop(query_pairs);
		url
	}

	fn http_method(&self) -> HttpMethod {
		HttpMethod::Get
	}
}

/// Each channel in the output data
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Channel {
	/// Channel language
	pub broadcaster_language: String,

	/// Display name
	pub display_name: String,

	/// Game id
	pub game_id: String,

	/// Channel id
	pub id: String,

	/// Live status
	pub is_live: bool,

	/// Tag IDs that apply to the stream.
	/// Note: Category tags are not returned
	pub tag_ids: Vec<String>,

	/// Thumbnail url
	pub thumbnail_url: String,

	/// Title
	pub title: String,

	/// UTC timestamp for stream start
	/// Live streams only.
	#[serde(with = "crate::util::utc_date_time")]
	pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}
