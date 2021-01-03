//! Games analytics request

// Imports
use super::ReportType;
use crate::{helix_url, HelixRequest, HttpMethod};

/// Games analytics request
///
/// This request uses the `/analytics/games` path
/// to get a url to download game analytics reports.
///
/// # Examples
/// ```
/// # use twitch_helix::request::analytics::games::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new();
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/analytics/games");
/// assert_eq!(url.query(), Some(""));
/// ```
#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Request {
	/// Cursor for forward pagination
	pub after: Option<String>,

	/// Ending date/time for returned reports
	pub ended_at: Option<chrono::DateTime<chrono::Utc>>,

	/// Game id
	pub game_id: Option<String>,

	/// Maximum number of objects to return
	pub first: Option<usize>,

	/// Starting date for returned reports
	pub started_at: Option<chrono::DateTime<chrono::Utc>>,

	/// Type of analytics report
	pub report_type: Option<ReportType>,
}

impl Request {
	/// Creates a new request with all parameters default
	#[must_use]
	pub fn new() -> Self {
		Self {
			after: None,
			ended_at: None,
			game_id: None,
			first: None,
			started_at: None,
			report_type: None,
		}
	}

	/// Sets the cursor for forward pagination
	#[must_use]
	pub fn with_after(self, after: String) -> Self {
		Self { after: Some(after), ..self }
	}

	/// Sets the ending date/time for returned reports
	#[must_use]
	pub fn with_ended_at(self, ended_at: chrono::DateTime<chrono::Utc>) -> Self {
		Self {
			ended_at: Some(ended_at),
			..self
		}
	}

	/// Sets the game id
	#[must_use]
	pub fn with_game_id(self, game_id: String) -> Self {
		Self {
			game_id: Some(game_id),
			..self
		}
	}

	/// Sets the maximum number of objects to return
	#[must_use]
	pub fn with_first(self, first: usize) -> Self {
		Self { first: Some(first), ..self }
	}

	/// Sets the starting date for returned reports
	#[must_use]
	pub fn with_started_at(self, started_at: chrono::DateTime<chrono::Utc>) -> Self {
		Self {
			started_at: Some(started_at),
			..self
		}
	}

	/// Sets the type of analytics report
	#[must_use]
	pub fn with_report_type(self, report_type: ReportType) -> Self {
		Self {
			report_type: Some(report_type),
			..self
		}
	}
}

impl HelixRequest for Request {
	type Response = Vec<Report>;

	fn url(&self) -> url::Url {
		// Append all our arguments if they exist
		let mut url = helix_url!(analytics / games);

		{
			let mut query_pairs = url.query_pairs_mut();
			if let Some(after) = &self.after {
				query_pairs.append_pair("after", after);
			}
			if let Some(ended_at) = &self.ended_at {
				query_pairs.append_pair("ended_at", &ended_at.to_rfc3339());
			}
			if let Some(game_id) = &self.game_id {
				query_pairs.append_pair("game_id", game_id);
			}
			if let Some(first) = &self.first {
				query_pairs.append_pair("first", &first.to_string());
			}
			if let Some(started_at) = &self.started_at {
				query_pairs.append_pair("started_at", &started_at.to_rfc3339());
			}
			if let Some(report_type) = &self.report_type {
				let value = match report_type {
					ReportType::OverviewV1 => "overview_v1",
					ReportType::OverviewV2 => "overview_v2",
				};
				query_pairs.append_pair("type", value);
			}
		}
		url
	}

	fn http_method(&self) -> HttpMethod {
		HttpMethod::Get
	}
}

/// A Report
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Report {
	/// Game id
	pub game_id: String,

	/// Date range
	pub date_range: DateRange,

	/// Type of report
	pub report_type: ReportType,

	/// URL
	#[serde(rename = "URL")]
	pub url: String,
}

/// Data range
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DateRange {
	/// UTC timestamp for starting time
	pub started_at: chrono::DateTime<chrono::Utc>,

	/// UTC timestamp for ending time
	pub ended_at: chrono::DateTime<chrono::Utc>,
}
