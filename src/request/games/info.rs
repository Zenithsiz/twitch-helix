//! Game info request

// Imports
use crate::{helix_url, HelixRequest};
use reqwest as req;

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

impl Request {
	/// Finds the exact game requested given the response
	///
	/// Attempts to find an exact match in either the `id` or `name`
	/// fields of the channel, without considering case.
	#[must_use]
	pub fn game(&self, games: Vec<Game>) -> Option<Game> {
		// Check every game in the response
		for game in games {
			match self {
				Self::Id(id) => {
					if unicase::eq(id, &game.id) {
						return Some(game);
					}
				}
				Self::Name(name) => {
					if unicase::eq(name, &game.name) {
						return Some(game);
					}
				}
			}
		}

		// If we get here, no game was found
		None
	}

	/// Finds the exact game requested given the response by reference.
	///
	/// See [`Self::game`] for more information.
	#[must_use]
	pub fn game_ref<'a>(&self, games: &'a [Game]) -> Option<&'a Game> {
		// Check every game in the response
		for game in games {
			match self {
				Self::Id(id) => {
					if unicase::eq(id, &game.id) {
						return Some(game);
					}
				}
				Self::Name(name) => {
					if unicase::eq(name, &game.name) {
						return Some(game);
					}
				}
			}
		}

		// If we get here, no game was found
		None
	}
}

impl HelixRequest for Request {
	type Response = Vec<Game>;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(games);
		let mut query_pairs = url.query_pairs_mut();
		match self {
			Self::Id(id) => query_pairs.append_pair("id", id),
			Self::Name(name) => query_pairs.append_pair("name", name),
		};
		std::mem::drop(query_pairs);
		url
	}

	fn http_method(&self) -> req::Method {
		req::Method::GET
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
