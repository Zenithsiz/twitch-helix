//! Channel search request

// Imports
use crate::{helix_url, response::Pagination, HelixRequest};
use reqwest as req;

/// Channel search request
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
			query:     query.into(),
			first:     None,
			after:     None,
			live_only: None,
		}
	}
}

impl HelixRequest for Request {
	type Output = Output;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(search / channels);
		url.query_pairs_mut().append_pair("query", &self.query);
		url
	}

	fn http_method(&self) -> req::Method {
		req::Method::GET
	}
}

/// Channel search output
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Output {
	/// Response data
	data: Vec<Channel>,

	/// Page
	pagination: Pagination,
}

/// Each channel in the output data
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Channel {
	/// Channel language
	broadcaster_language: String,

	/// Display name
	display_name: String,

	/// Game id
	game_id: String,

	/// Channel id
	id: String,

	/// Live status
	is_live: bool,

	/// Tag IDs that apply to the stream.
	/// Note: Category tags are not returned
	tag_ids: Vec<String>,

	/// Thumbnail url
	thumbnail_url: String,

	/// Title
	title: String,

	/// UTC timestamp for stream start
	/// Live streams only.
	#[serde(deserialize_with = "deserialize_channel_start_at")]
	started_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Deserializer for [`Channel::started_at`]
fn deserialize_channel_start_at<'de, D>(deserializer: D) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	// Deserialize as a string
	let started_at = <String as serde::Deserialize>::deserialize(deserializer)?;

	// If it's empty, return `None`
	if started_at.is_empty() {
		return Ok(None);
	}

	// Else try to parse it as a `Utc`
	match started_at.parse() {
		Ok(started_at) => Ok(Some(started_at)),

		// On error, give an `invalid_value` error.
		Err(err) => Err(<D::Error as serde::de::Error>::invalid_value(
			serde::de::Unexpected::Str(&started_at),
			&format!("Unable to parse time as `DateTime<Utc>`: {}", err).as_str(),
		)),
	}
}
