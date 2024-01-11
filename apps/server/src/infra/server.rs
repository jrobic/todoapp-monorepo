use std::env::current_dir;
use std::sync::Arc;

use axum::routing;
use axum::{routing::get, Router};

use tokio::sync::broadcast::{channel, Sender};
use tower_http::services::ServeDir;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::repository::todo_repository::DynTodoRepository;

use super::controller;
use super::controller::todos_views_ctrl::UpdateTodoTmpl;
use super::repository;

#[derive(Clone)]
pub struct AppState {
	pub todo_repo: DynTodoRepository,
	pub tx: Arc<Sender<UpdateTodoTmpl>>,
}

impl AppState {
	pub fn broadcast_update_to_view(&self, update: UpdateTodoTmpl) {
		if self.tx.send(update).is_err() {
			info!(
				"Record with Id {} was created but nobody's listening to the stream!",
				"test"
			);
		}
	}
}

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
		);

	let assets_path = current_dir().unwrap().join("assets");

	let (tx, _rx) = channel::<UpdateTodoTmpl>(10);

	let app_state = AppState {
		todo_repo: todo_repo.clone(),
		tx: Arc::new(tx),
	};

	let views_router = Router::new()
		.route(
			"/hello",
			routing::get(controller::common_views_ctrl::render_hello_ctrl),
		)
		.route(
			"/",
			routing::get(controller::todos_views_ctrl::render_index_ctrl),
		)
		.route(
			"/stream",
			routing::get(controller::todos_views_ctrl::stream_ctrl),
		)
		.route(
			"/list_todos",
			routing::get(controller::todos_views_ctrl::list_todos_ctrl),
		)
		.route(
			"/create_todo",
			routing::post(controller::todos_views_ctrl::create_todo_ctrl),
		)
		.route(
			"/mark_as_done/:id",
			routing::post(controller::todos_views_ctrl::mark_as_done_todo_ctrl),
		)
		.route(
			"/mark_as_undone/:id",
			routing::post(controller::todos_views_ctrl::mark_as_undone_todo_ctrl),
		)
		.route(
			"/remove_todo/:id",
			routing::delete(controller::todos_views_ctrl::delete_todo_ctrl),
		)
		// .route(
		// 	"/clear_all_completed_todos",
		// 	routing::post(controller::todos_views_ctrl::clear_all_completed_todos_ctrl),
		// )
		.route(
			"/todos_sse",
			get(controller::todos_views_ctrl::todos_stream),
		)
		.nest_service("/assets", ServeDir::new(assets_path));

	let app = Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
		.merge(todo_router)
		.merge(views_router)
		.with_state(app_state)
		// .with_state(Arc::new(tx))
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer());

	#[cfg(not(debug_assertions))]
	let app = {
		use tower_http::compression::CompressionLayer;

		let compression_layer = CompressionLayer::new()
			.br(true)
			.deflate(true)
			.gzip(true)
			.zstd(true)
			.compress_when(|_, _, _: &_, _: &_| true);

		app.layer(compression_layer)
	};

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
