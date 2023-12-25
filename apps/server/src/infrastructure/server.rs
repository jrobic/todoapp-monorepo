use axum::{routing::get, Router};

use super::controller;

pub fn create_server() -> Router {
	Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer())
}
