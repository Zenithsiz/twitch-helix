//! Game info request

// Imports
use crate::{helix_url, HelixRequest, HttpMethod};

/// Game info request
///
/// This request uses the `/games` path
/// to get info about a game, by either
/// it's id, or it's name.
///
/// Response is a list of `[Game]s`.
///
/// #Examples
/// Search by id:
/// ```
/// # use twitch_helix::request::games::info::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::Id("my-game-id".to_string());
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/games");
/// assert_eq!(url.query(), Some("id=my-game-id"));
/// ```
///
/// Search by name:
/// ```
/// # use twitch_helix::request::games::info::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::Name("my-game-name".to_string());
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/games");
/// assert_eq!(url.query(), Some("name=my-game-name"));
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Request {
	/// Search by id
	Id(String),

	/// Search by name
	Name(String),
}

impl HelixRequest for Request {
	type Response = Vec<Game>;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(games);
		{
			let mut query_pairs = url.query_pairs_mut();
			match self {
				Self::Id(id) => query_pairs.append_pair("id", id),
				Self::Name(name) => query_pairs.append_pair("name", name),
			};
		}
		url
	}

	fn http_method(&self) -> HttpMethod {
		HttpMethod::Get
	}
}

/// Each game in the output data
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
	/// Game's box art
	pub box_art_url: String,

	/// Game id
	pub id: String,

	/// Game name
	pub name: String,
}
