use axum::{
	extract::{Path, Query, State},
	http::{HeaderMap, StatusCode},
	Json,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
	domain::entity::todo::Todo,
	infra::{
		api_response::{ApiResponse, ApiResponseData},
		server::AppState,
	},
	usecase::{
		create_todo_usecase::{self, CreateTodoParams},
		delete_todo_usecase, get_all_todos_usecase, mark_as_done_todo_usecase,
	},
};

use super::helper::extract_status_from_header;

#[utoipa::path(
	tag = "Todo",
	post,
	path = "/api/todos",
	request_body = CreateTodoParams,
	responses(
		(status = 201, description = "Todo item created successfully", body = ApiResponseTodo),
		(status = 409, description = "Todo already exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn create_todo_ctrl(
	State(app_state): State<AppState>,
	Json(params): Json<CreateTodoParams>,
) -> ApiResponse<Todo> {
	let create_todo_usecase = create_todo_usecase::CreateTodoUsecase::new(&app_state.todo_repo);

	let todo = create_todo_usecase.exec(params).await?;

	Ok(ApiResponseData::success_with_data(
		todo,
		StatusCode::CREATED,
	))
}

#[derive(Deserialize, IntoParams, Clone, Debug)]
#[into_params(parameter_in = Query)]
pub struct GetAllTodosQuery {
	pub status: Option<String>,
}

#[utoipa::path(
	tag = "Todo",
	get,
	path = "/api/todos",
	params(GetAllTodosQuery),
	responses(
		(status = 200, description = "Todo items retrieved successfully", body = ApiResponseListTodos),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]

pub async fn get_all_todos_ctrl(
	State(app_state): State<AppState>,
	query: Query<GetAllTodosQuery>,
) -> ApiResponse<Vec<Todo>> {
	let get_all_todos_usecase =
		get_all_todos_usecase::GetAllTodosUsecase::new(&app_state.todo_repo);

	let todos = get_all_todos_usecase.exec(query.status.as_ref()).await?;

	Ok(ApiResponseData::success_with_data(todos, StatusCode::OK))
}

#[utoipa::path(
	tag = "Todo",
	delete,
	path = "/api/todos/{id}",
	params(
		("id" = String, Path, description = "Todo item id"),
	),
	responses(
		(status = 204, description = "Todo item deleted successfully"),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn delete_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<String>,
) -> ApiResponse<()> {
	let delete_todo_usecase = delete_todo_usecase::DeleteTodoUsecase::new(&app_state.todo_repo);

	delete_todo_usecase.exec(id).await?;

	Ok(ApiResponseData::status_code(StatusCode::NO_CONTENT))
}

#[utoipa::path(
	tag = "Todo",
	patch,
	path = "/api/todos/{id}/mark_as_done",
	params(
		("id" = String, Path, description = "Todo item id"),
	),
	responses(
		(status = 200, description = "Todo item marked as done successfully", body = ApiResponseTodo),
		(status = 422, description = "Todo item not exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn mark_as_done_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<String>,
) -> ApiResponse<Todo> {
	let mark_as_done_usecase =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&app_state.todo_repo);

	let todo = mark_as_done_usecase.exec(id, true).await?;

	Ok(ApiResponseData::success_with_data(todo, StatusCode::OK))
}

#[utoipa::path(
	tag = "Todo",
	patch,
	path = "/api/todos/{id}/mark_as_undone",
	params(
		("id" = String, Path, description = "Todo item id"),
	),
	responses(
		(status = 200, description = "Todo item marked as undone successfully", body = ApiResponseTodo),
		(status = 422, description = "Todo item not exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn mark_as_undone_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<String>,
) -> ApiResponse<Todo> {
	let mark_as_done_usecase =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&app_state.todo_repo);

	let todo = mark_as_done_usecase.exec(id, false).await?;

	Ok(ApiResponseData::success_with_data(todo, StatusCode::OK))
}

pub async fn count_todos_ctrl(
	State(app_state): State<AppState>,
	headers: HeaderMap,
) -> ApiResponse<i64> {
	let status: Option<String> = extract_status_from_header(headers);

	let count_todos_usecase =
		crate::usecase::count_todos_usecase::CountTodosUsecase::new(&app_state.todo_repo);

	let count = count_todos_usecase.exec(status.as_ref()).await;

	Ok(ApiResponseData::success_with_data(count, StatusCode::OK))
}
