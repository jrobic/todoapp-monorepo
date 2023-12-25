use axum::{http::StatusCode, response::IntoResponse, Json};

#[utoipa::path(
	tag = "Core",
	get,
	path = "/health",
	responses(
		(status = 200, description = "OK", body = Health),
	)
)]
pub async fn health() -> impl IntoResponse {
	let health_usecase = crate::usecase::health_usecase::HealthUsecase::new();

	let heatlh = health_usecase.get_health();

	(StatusCode::OK, Json(heatlh))
}
