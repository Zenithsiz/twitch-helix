//! Analytics requests

// Modules
pub mod extensions;
pub mod games;

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
