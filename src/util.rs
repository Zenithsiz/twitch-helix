//! Utilities

/// UTC date-times [de]serializer
pub mod utc_date_time {
	// Imports
	use chrono::{DateTime, Utc};
	use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

	/// UTC date-times deserializer
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
	pub fn serialize<S>(date_time: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match date_time {
			Some(date_time) => date_time.serialize(serializer),
			None => "".serialize(serializer),
		}
	}

	#[cfg(test)]
	mod test {
		// Imports
		use super::*;
		use chrono::{Datelike, Timelike};

		#[test]
		fn deserialize_some() {
			let mut deserializer = serde_json::Deserializer::from_str("\"2020-07-23T14:49:33Z\"");
			let date_time = deserialize(&mut deserializer)
				.expect("Unable to parse utc date-time")
				.expect("Parsed no utc time-date from a non-empty string");
			assert_eq!(date_time.year(), 2020);
			assert_eq!(date_time.month(), 7);
			assert_eq!(date_time.day(), 23);
			assert_eq!(date_time.hour(), 14);
			assert_eq!(date_time.minute(), 49);
			assert_eq!(date_time.second(), 33);
		}

		#[test]
		fn deserialize_none() {
			let mut deserializer = serde_json::Deserializer::from_str("\"\"");
			let date_time = deserialize(&mut deserializer).expect("Unable to parse utc date-time");
			assert!(date_time.is_none());
		}

		#[test]
		fn serialize_some() {
			use chrono::Utc;
			let now = Utc::now();
			let mut buf = vec![];
			let mut serializer = serde_json::Serializer::new(&mut buf);
			serialize(&Some(now), &mut serializer).expect("Unable to serialize utc date-time");

			let mut deserializer = serde_json::Deserializer::from_slice(&buf);
			let date_time = deserialize(&mut deserializer).expect("Unable to parse utc date-time");

			assert_eq!(date_time, Some(now));
		}

		#[test]
		fn serialize_none() {
			let mut buf = vec![];
			let mut serializer = serde_json::Serializer::new(&mut buf);
			serialize(&None, &mut serializer).expect("Unable to serialize utc date-time");

			let mut deserializer = serde_json::Deserializer::from_slice(&buf);
			let date_time = deserialize(&mut deserializer).expect("Unable to parse utc date-time");

			assert_eq!(date_time, None);
		}
	}
}
