use axum::{
	http::{header, StatusCode, Uri},
	response::IntoResponse,
	Json,
};
use rust_embed::RustEmbed;
use tracing::instrument;

#[utoipa::path(
	tag = "Core",
	get,
	path = "/health",
	responses(
		(status = 200, description = "OK", body = Health),
	)
)]
#[instrument(name = "Health Controller")]
pub async fn health() -> impl IntoResponse {
	let health_usecase = crate::usecase::health_usecase::HealthUsecase::new();

	let heatlh = health_usecase.get_health();

	(StatusCode::OK, Json(heatlh))
}

#[derive(RustEmbed)]
#[folder = "assets/"]
#[include = "*.css"]
#[include = "*.js"]
pub struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
	T: Into<String>,
{
	fn into_response(self) -> axum::response::Response {
		let path = self.0.into();

		match Asset::get(path.as_str()) {
			Some(content) => {
				let mime = mime_guess::from_path(path).first_or_octet_stream();
				([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
			},
			None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
		}
	}
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
	let mut path = uri.path().trim_start_matches('/').to_string();

	if path.starts_with("assets/") {
		path = path.replace("assets/", "");
	}

	StaticFile(path)
}
