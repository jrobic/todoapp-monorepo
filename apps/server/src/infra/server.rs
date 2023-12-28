use std::future::ready;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::{MatchedPath, Request};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::routing;
use axum::{routing::get, Router};

use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
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
			"/todos",
			routing::get(controller::todo_ctrl::get_all_todos_ctrl)
				.post(controller::todo_ctrl::create_todo_ctrl),
		)
		.route(
			"/todos/:id",
			routing::delete(controller::todo_ctrl::delete_todo_ctrl),
		)
		.route(
			"/todos/:id/mark_as_done",
			routing::patch(controller::todo_ctrl::mark_as_done_todo_ctrl),
		)
		.route(
			"/todos/:id/mark_as_undone",
			routing::patch(controller::todo_ctrl::mark_as_undone_todo_ctrl),
		)
		.with_state(todo_repo);

	Router::new()
		.route("/health", get(controller::common_ctrl::health))
		.merge(SwaggerUi::new("/swagger").url("/openapi.json", doc))
		.merge(todo_router)
		.route(
			"/slow",
			get(|| async {
				tokio::time::sleep(Duration::from_secs(1)).await;
			}),
		)
		.route_layer(middleware::from_fn(track_metrics))
		.fallback(controller::catchers_ctrl::not_found_ctrl)
		.layer(super::tracing::add_tracing_layer())
}

pub fn create_metrics_server() -> Router {
	let recorder_handle = setup_metrics_recorder();

	Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

fn setup_metrics_recorder() -> PrometheusHandle {
	const EXPONENTIAL_SECONDS: &[f64] = &[
		0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
	];

	PrometheusBuilder::new()
		.set_buckets_for_metric(
			Matcher::Full("http_requests_duration_seconds".to_string()),
			EXPONENTIAL_SECONDS,
		)
		.unwrap()
		.install_recorder()
		.unwrap()
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
	let start = Instant::now();
	let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
		matched_path.as_str().to_owned()
	} else {
		req.uri().path().to_owned()
	};
	let method = req.method().clone();

	let response = next.run(req).await;

	let latency = start.elapsed().as_secs_f64();
	let status = response.status().as_u16().to_string();

	let labels = [
		("method", method.to_string()),
		("path", path),
		("status", status),
	];

	metrics::counter!("http_requests_total", &labels).increment(1);
	metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

	response
}
