//! Pagination for each response

/// Response pagination
///
/// Represents the current page from the request.
/// May be fed into some requests to get the next page,
/// as a linked-list.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Pagination {
	/// String
	String(String),

	/// Object with optional cursor
	WithCursor {
		/// Optional cursor when pagination is an object.
		/// If not present, this may be the last page.
		cursor: Option<String>,
	},
}

impl Pagination {
	/// Gets this pagination as a cursor
	#[must_use]
	pub fn as_cursor(&self) -> Option<&str> {
		match self {
			Self::WithCursor { cursor } => cursor.as_deref(),
			_ => None,
		}
	}
}
