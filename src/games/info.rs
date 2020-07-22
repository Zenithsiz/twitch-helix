//! Get game info request

// Imports
use crate::{helix_url, response::Pagination, HelixRequest};
use reqwest as req;

/// Game info request
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Request {
	/// Search by id
	Id(String),

	/// Search by name
	Name(String),
}

impl Request {
	/// Returns the specific game found by this request's
	/// response
	///
	/// It attempts to exact match any games in the output
	/// with the request, if found, it is returned, else
	/// `None` is returned.
	#[must_use]
	pub fn game<'a>(&self, output: &'a Output) -> Option<&'a Game> {
		// Check every game in the response
		for game in &output.data {
			match self {
				Self::Id(id) => {
					if unicase::eq(id, &game.id) {
						return Some(game);
					}
				},
				Self::Name(name) => {
					if unicase::eq(name, &game.name) {
						return Some(game);
					}
				},
			}
		}

		// If we get here, no game was found
		None
	}
}

impl HelixRequest for Request {
	type Output = Output;

	fn url(&self) -> url::Url {
		let mut url = helix_url!(search / channels);
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

/// Game info output
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Output {
	/// Response data
	pub data: Vec<Game>,

	/// Page
	pub pagination: Pagination,
}

/// Each game in the output data
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Game {
	/// Game's box art
	box_art_url: String,

	/// Game id
	id: String,

	/// Game name
	name: String,
}
