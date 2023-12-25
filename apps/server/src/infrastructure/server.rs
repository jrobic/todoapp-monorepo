use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::controller;

pub fn create_server() -> Router {
	let doc = super::api_doc::ApiDoc::openapi();

	Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer())
}
