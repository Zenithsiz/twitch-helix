//! Helix URL helpers

// Modules
#[cfg(test)]
mod test;

/// Creates a `url::Url` to a helix page
#[macro_export]
macro_rules! helix_url {
	() => { "https://api.twitch.tv/helix/" };
	( $($path:ident)/+ ) => {{
		// Build the url by appending `path/` successively
		let url = concat!(helix_url!(), $(stringify!($path), "/",)+);

		// Remove the last `/`
		let url = url.trim_end_matches('/');

		// And parse it as a url
		url::Url::parse(url).expect("Unable to parse known url")
	}};
}

/// Creates a `url::Url` to a oauth page
#[macro_export]
macro_rules! oauth_url {
	() => { "https://id.twitch.tv/" };
	( $($path:ident)/+ ) => {{
		// Build the url by appending `path/` successively
		let url = concat!(oauth_url!(), $(stringify!($path), "/",)+);

		// Remove the last `/`
		let url = url.trim_end_matches('/');

		// And parse it as a url
		url::Url::parse(url).expect("Unable to parse known url")
	}};
}
