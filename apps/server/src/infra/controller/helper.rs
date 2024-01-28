use std::collections::HashMap;

use axum::http::HeaderMap;
use url::Url;

pub fn extract_status_from_header(headers: HeaderMap) -> Option<String> {
	let current_url = headers.get("hx-current-url").or(headers.get("referer"));

	current_url.and_then(|url| {
		let hash_query: HashMap<_, _> =
			Url::parse(url.to_str().unwrap()).unwrap().query_pairs().into_owned().collect();

		hash_query.get("status").map(|s| s.to_string())
	})
}
