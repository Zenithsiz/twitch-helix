//! Extension analytics request

// Imports
use crate::{helix_url, HelixRequest, HttpMethod};

/// Extension analytics request
///
/// This request uses the `/analytics/extension` path
/// to get a url to download analytics reports.
///
/// # Examples
/// ```
/// # use twitch_helix::request::analytics::extensions::Request;
/// # use twitch_helix::HelixRequest;
/// let mut request = Request::new();
///
/// let url = request.url();
/// assert_eq!(url.host_str(), Some("api.twitch.tv"));
/// assert_eq!(url.path(), "/helix/analytics/extensions");
/// assert_eq!(url.query(), Some(""));
/// ```
#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Request {
	/// Cursor for forward pagination
	after: Option<String>,

	/// Ending date/time for returned reports
	ended_at: Option<chrono::DateTime<chrono::Utc>>,

	/// Extension id
	extension_id: Option<String>,

	/// Maximum number of objects to return
	first: Option<usize>,

	/// Starting date for returned reports
	started_at: Option<chrono::DateTime<chrono::Utc>>,

	/// Type of analytics report
	report_type: Option<ReportType>,
}

/// A report type
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReportType {
	/// Overview V1
	#[serde(rename = "overview_v1")]
	OverviewV1,

	/// Overview V2
	#[serde(rename = "overview_v2")]
	OverviewV2,
}

impl Request {
	/// Creates a new request with all parameters default
	#[must_use]
	pub fn new() -> Self {
		Self {
			after: None,
			ended_at: None,
			extension_id: None,
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

	/// Sets the extension id
	#[must_use]
	pub fn with_extension_id(self, extension_id: String) -> Self {
		Self {
			extension_id: Some(extension_id),
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

	/// Finds the exact extension requested given the response
	///
	/// Attempts to find an exact match in the `display_name`
	/// field of the channel, without considering case.
	#[must_use]
	pub fn extension(&self, channels: Vec<Report>) -> Option<Report> {
		let extension_id = self.extension_id.as_ref()?;

		// Check every channel in the response
		for channel in channels {
			if unicase::eq(extension_id, &channel.extension_id) {
				return Some(channel);
			}
		}

		// If we get here, no channel was found
		None
	}

	/// Finds the exact extension requested given the response by reference.
	///
	/// See [`Self::extension`] for more information.
	#[must_use]
	pub fn extension_ref<'a>(&self, channels: &'a [Report]) -> Option<&'a Report> {
		let extension_id = self.extension_id.as_ref()?;

		// Check every channel in the response
		for channel in channels {
			if unicase::eq(extension_id, &channel.extension_id) {
				return Some(channel);
			}
		}

		// If we get here, no channel was found
		None
	}
}

impl HelixRequest for Request {
	type Response = Vec<Report>;

	fn url(&self) -> url::Url {
		// Append all our arguments if they exist
		let mut url = helix_url!(analytics / extensions);

		{
			let mut query_pairs = url.query_pairs_mut();
			if let Some(after) = &self.after {
				query_pairs.append_pair("after", after);
			}
			if let Some(ended_at) = &self.ended_at {
				query_pairs.append_pair("ended_at", &ended_at.to_rfc3339());
			}
			if let Some(extension_id) = &self.extension_id {
				query_pairs.append_pair("extension_id", extension_id);
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
	/// Extension id
	pub extension_id: String,

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
