use axum::{routing, Router};

use super::{controller, server::AppState};

pub fn api_routes() -> Router<AppState> {
	Router::new()
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
		.route(
			"/api/todos/count",
			routing::get(controller::todo_ctrl::count_todos_ctrl),
		)
}

pub fn views_routes() -> Router<AppState> {
	Router::new()
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
			"/count_todos",
			routing::get(controller::todos_views_ctrl::count_todos_ctrl),
		)
		.route(
			"/todos_sse",
			routing::get(controller::todos_views_ctrl::todos_stream),
		)
		.route(
			"/assets/*file",
			routing::get(controller::common_ctrl::static_handler),
		)
}
