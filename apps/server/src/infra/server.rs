use std::sync::Arc;

use axum::http::{HeaderName, Method};
use axum::routing;
use axum::{routing::get, Router};

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tokio::sync::broadcast::{channel, Sender};
use tower_http::cors::{self, CorsLayer};

use utoipa::OpenApi;

use crate::domain::repository::todo_repository::DynTodoRepository;

use super::controller::todos_views_ctrl::UpdateTodoTmpl;
use super::pg::create_pg_pool;
use super::repository;
use super::{controller, routes};

#[derive(Clone)]
pub struct AppState {
	pub todo_repo: DynTodoRepository,
	pub tx: Arc<Sender<UpdateTodoTmpl>>,
}

impl AppState {
	pub fn broadcast_update_to_view(&self, update: UpdateTodoTmpl) {
		if self.tx.send(update).is_err() {
			tracing::info!(
				"Record with Id {} was created but nobody's listening to the stream!",
				"test"
			);
		}
	}
}

pub async fn create_server() -> Router {
	let tracing_enabled: bool = std::env::var("TRACING").unwrap_or_else(|_| "0".to_string()) == "1";
	let inmemory_mode = std::env::var("INMEMORY_MODE").unwrap_or_else(|_| "0".to_string()) == "1";

	let doc: utoipa::openapi::OpenApi = super::api_doc::ApiDoc::openapi();

	let pg_pool = create_pg_pool().await;

	let todo_repo: DynTodoRepository = match inmemory_mode {
		true => Arc::new(repository::todo_inmemory_repo::TodoInMemoryRepository::new()),
		false => Arc::new(repository::todo_pg_repo::TodoPgRepository::new(pg_pool)),
	};

	let (tx, _rx) = channel::<UpdateTodoTmpl>(10);

	let app_state = AppState {
		todo_repo,
		tx: Arc::new(tx),
	};

	let cors = CorsLayer::new()
		.allow_methods([
			Method::GET,
			Method::POST,
			Method::DELETE,
			Method::PATCH,
			Method::PUT,
		])
		.allow_origin(cors::Any)
		.allow_headers(vec![
			HeaderName::from_static("authorization"),
			HeaderName::from_static("content-type"),
		]);

	let mut app = Router::new()
		.merge(routes::api_routes())
		.merge(routes::views_routes())
		.with_state(app_state)
		// .with_state(Arc::new(tx))
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.route("/api/openapi", routing::get(doc.clone().to_json().unwrap()))
		.route("/health", get(controller::common_ctrl::health))
		.layer(cors)
		.layer(super::tracing::add_fmt_layer());

	if tracing_enabled {
		app = app
			.layer(OtelInResponseLayer)
			//start OpenTelemetry trace on incoming request
			.layer(OtelAxumLayer::default());
	}

	#[cfg(not(debug_assertions))]
	let app: Router = {
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
		use std::env::current_dir;
		use utoipa_swagger_ui::SwaggerUi;

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

		app.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
			.layer(livereload)
	};

	app
}
