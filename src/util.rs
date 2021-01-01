//! Utilities

/// UTC date-times [de]serializer
pub mod utc_date_time {
	// Imports
	use chrono::{DateTime, Utc};
	use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

	/// UTC date-times deserializer
	///
	/// # Example
	/// ```
	/// # use twitch_helix::util::deserialize_utc_date_time;
	/// use chrono::{Datelike, Timelike};
	/// let mut deserializer = serde_json::Deserializer::from_str("\"2020-07-23T14:49:33Z\"");
	/// let res = deserialize_utc_date_time(&mut deserializer)
	///   .expect("Unable to parse utc date-time")
	///   .expect("Parsed no utc time-date from a non-empty string");
	/// assert_eq!(res.year(), 2020);
	/// assert_eq!(res.month(), 07);
	/// assert_eq!(res.day(), 23);
	/// assert_eq!(res.hour(), 14);
	/// assert_eq!(res.minute(), 49);
	/// assert_eq!(res.second(), 33);
	/// ```
	///
	/// Deserializing an empty value returns `None`.
	/// ```
	/// # use twitch_helix::util::utc_date_time::deserialize;
	/// use chrono::{Datelike, Timelike};
	/// let mut deserializer = serde_json::Deserializer::from_str("");
	/// let res = deserialize(&mut deserializer)
	///   .expect("Unable to parse utc date-time");
	/// assert!(res.is_none());
	/// ```
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Deserialize as a string
		let started_at = String::deserialize(deserializer)?;

		// If it's empty, return `None`
		if started_at.is_empty() {
			return Ok(None);
		}

		// Else try to parse it as a `Utc`
		match started_at.parse() {
			Ok(started_at) => Ok(Some(started_at)),

			// On error, give an `invalid_value` error.
			Err(err) => Err(<D::Error as de::Error>::invalid_value(
				de::Unexpected::Str(&started_at),
				&format!("Unable to parse time as `DateTime<Utc>`: {}", err).as_str(),
			)),
		}
	}

	/// UTC date-times serializer
	///
	/// # Example
	/// ```
	/// # use twitch_helix::util::utc_date_time::{serialize, deserialize};
	/// use chrono::Utc;
	/// let now = Utc::now();
	/// let mut buf = String::new();
	/// serialize(&Some(now), serde_json::Serializer::new(&mut buf))
	///   .expect("Unable to serialize utc date-time");
	/// let mut deserializer = serde_json::Deserializer::from_str(buf);
	/// let res = deserialize(&mut deserializer)
	///   .expect("Unable to parse utc date-time");
	/// assert_eq!(res, Some(now));
	/// ```
	pub fn serialize<S>(date_time: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match date_time {
			Some(date_time) => date_time.serialize(serializer),
			None => "".serialize(serializer),
		}
	}
}
