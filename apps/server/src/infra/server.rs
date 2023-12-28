use std::env::current_dir;
use std::sync::Arc;

use axum::routing;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
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

	let assets_path = current_dir().unwrap().join("assets");

	let views_router = Router::new()
		.route(
			"/hello",
			routing::get(controller::common_views_ctrl::render_hello_ctrl),
		)
		.route(
			"/",
			routing::get(controller::todos_views_ctrl::render_index_ctrl),
		)
		.nest_service("/assets", ServeDir::new(assets_path));

	let app = Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
		.merge(todo_router)
		.merge(views_router)
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer());

	#[cfg(debug_assertions)]
	let app = {
		use notify::Watcher;

		let livereload = tower_livereload::LiveReloadLayer::new()
			.request_predicate(|req: &axum::http::Request<axum::body::Body>| {
				!req.headers().contains_key("hx-request")
			})
			.reload_interval(std::time::Duration::from_millis(500));
		let reloader = livereload.reloader();
		let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();

		let assets_path = current_dir().unwrap().join("assets");
		let templates_path = current_dir().unwrap().join("src/templates");

		watcher.watch(&assets_path, notify::RecursiveMode::Recursive).unwrap();
		watcher.watch(&templates_path, notify::RecursiveMode::Recursive).unwrap();

		tracing::info!("Reloading!");

		app.layer(livereload)
	};

	app
}
