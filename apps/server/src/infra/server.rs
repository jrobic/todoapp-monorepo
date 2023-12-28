use std::sync::Arc;

use axum::routing;
use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::repository::todo_repository::DynTodoRepository;

use super::controller;
use super::repository;

pub fn create_server() -> Router {
	let doc = super::api_doc::ApiDoc::openapi();

	let todo_repo: DynTodoRepository =
		Arc::new(repository::todo_inmemory_repo::TodoInMemoryRepository::new());

	let todo_router = Router::new()
		.route(
			"/api/todos",
			routing::get(controller::todo_ctrl::get_all_todos_ctrl)
				.post(controller::todo_ctrl::create_todo_ctrl),
		)
		.route(
			"/api/todos/:id",
			routing::delete(controller::todo_ctrl::delete_todo_ctrl),
		)
		.route(
			"/api/todos/:id/mark_as_done",
			routing::patch(controller::todo_ctrl::mark_as_done_todo_ctrl),
		)
		.route(
			"/api/todos/:id/mark_as_undone",
			routing::patch(controller::todo_ctrl::mark_as_undone_todo_ctrl),
		)
		.with_state(todo_repo);

	let views_router = Router::new().route(
		"/hello",
		routing::get(controller::common_views_ctrl::render_hello_ctrl),
	);

	Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
		.merge(todo_router)
		.merge(views_router)
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer())
}
