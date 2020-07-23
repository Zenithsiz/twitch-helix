//! URL Helpers
//!
//! This module contains several macros that help build a url to
//! either the twitch `OAuth` or `Helix` service.
//!
//! These macros automatically return a [`url::Url`], which although
//! fallible in it's parsing, is infallible via these macros.
//!
//! This is due to them only accepting certain kinds of input, which
//! allows them to reason that a url must be well formatted.
//!
//! If you ever encounter a panic with message `"Unable to parse known url"`,
//! report it as a bug, as it is intended for this to always succeed.

/// Creates a url to a helix page
///
/// # Usage
/// This macro takes any number of arguments, identifiers,
/// separated by slashes, `/`, to build the full url.
/// There may _not_ exist a trailing `/`.
///
/// # Examples
/// The following examples demonstrates how to use the macro
/// ```
/// # use twitch_helix::helix_url;
/// assert_eq!( helix_url!().as_str(), "https://api.twitch.tv/helix" );
/// assert_eq!( helix_url!(my/page).as_str(), "https://api.twitch.tv/helix/my/page" );
/// ```
///
/// The following example doesn't compile because of the trailing `/`
/// ```compile_fail
/// # use twitch_helix::helix_url;
/// let my_url = helix_url!(my/page/); // Error: trailing `/`
/// ```
#[macro_export]
macro_rules! helix_url {
	(@base) => { "https://api.twitch.tv/helix" };
	( $($path:ident)/ * ) => {{
		// Build the url by appending `/path` successively to the base url
		let url = concat!(helix_url!(@base), $("/", stringify!($path),)*);

		// And parse it as a url
		url::Url::parse(url).expect("Unable to parse known url")
	}};
}

/// Creates a url to a oauth page
///
/// # Usage
/// This macro takes any number of arguments, identifiers,
/// separated by slashes, `/`, to build the full url.
/// There may _not_ exist a trailing `/`.
///
/// # Caveats
/// Due to how `url` works, the base oauth url path will contain a trailing
/// slash, `/` at the end.
///
/// # Examples
/// The following examples demonstrates how to use the macro
/// ```
/// # use twitch_helix::oauth_url;
/// assert_eq!( oauth_url!().as_str(), "https://id.twitch.tv/" );
/// assert_eq!( oauth_url!(my/page).as_str(), "https://id.twitch.tv/my/page" );
/// ```
///
/// The following example doesn't compile because of the trailing `/`
/// ```compile_fail
/// # use twitch_helix::oauth_url;
/// let my_url = oauth_url!(my/page/); // Error: trailing `/`
/// ```
#[macro_export]
macro_rules! oauth_url {
	(@base) => { "https://id.twitch.tv" };
	( $($path:ident)/ * ) => {{
		// Build the url by appending `/path` successively
		let url = concat!(oauth_url!(@base), $("/", stringify!($path),)*);

		// And parse it as a url
		url::Url::parse(url).expect("Unable to parse known url")
	}};
}
