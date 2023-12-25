use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(serde::Serialize)]
pub struct ErrorResponse {
	pub message: String,
}

pub async fn not_found_ctrl() -> impl IntoResponse {
	let error_response = ErrorResponse {
		message: String::from("Not Found"),
	};

	(StatusCode::NOT_FOUND, Json(error_response))
}
