use axum::{
	extract::{Path, Query, State},
	http::StatusCode,
	Json,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
	domain::{entity::todo::Todo, repository::todo_repository::DynTodoRepository},
	infra::api_response::{ApiResponse, ApiResponseData},
	usecase::{
		create_todo_usecase::{self, CreateTodoParams},
		delete_todo_usecase, get_all_todos_usecase, mark_as_done_todo_usecase,
		search_todos_usecase,
	},
};

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
	State(todo_repo): State<DynTodoRepository>,
	Json(params): Json<CreateTodoParams>,
) -> ApiResponse<Todo> {
	let create_todo_usecase = create_todo_usecase::CreateTodoUsecase::new(&todo_repo);

	let todo = create_todo_usecase.exec(params).await?;

	Ok(ApiResponseData::success_with_data(
		todo,
		StatusCode::CREATED,
	))
}

#[derive(Deserialize, IntoParams, Clone, Debug)]
#[into_params(parameter_in = Query)]
pub struct SearchTodosQuery {
	pub search_term: Option<String>,
}

#[utoipa::path(
	tag = "Todo",
	get,
	path = "/api/todos",
	params(SearchTodosQuery),
	responses(
		(status = 200, description = "Todo items retrieved successfully", body = ApiResponseListTodos),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]

pub async fn get_all_todos_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	query: Query<SearchTodosQuery>,
) -> ApiResponse<Vec<Todo>> {
	dbg!(query.clone());

	let search_term = query.search_term.clone();

	match search_term {
		Some(st) => {
			let search_todos_usecase = search_todos_usecase::SearchTodosUsecase::new(&todo_repo);
			let todos = search_todos_usecase.exec(st).await?;

			Ok(ApiResponseData::success_with_data(todos, StatusCode::OK))
		},
		None => {
			let get_all_todos_usecase = get_all_todos_usecase::GetAllTodosUsecase::new(&todo_repo);

			let todos = get_all_todos_usecase.exec().await?;

			Ok(ApiResponseData::success_with_data(todos, StatusCode::OK))
		},
	}
}

#[utoipa::path(
	tag = "Todo",
	delete,
	path = "/api/todos/{id}",
	params(
		("id" = Uuid, Path, description = "Todo item id"),
	),
	responses(
		(status = 204, description = "Todo item deleted successfully"),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn delete_todo_ctrl(
	State(_todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> ApiResponse<()> {
	let delete_todo_usecase = delete_todo_usecase::DeleteTodoUsecase::new(&_todo_repo);

	delete_todo_usecase.exec(id).await?;

	Ok(ApiResponseData::status_code(StatusCode::NO_CONTENT))
}

#[utoipa::path(
	tag = "Todo",
	patch,
	path = "/api/todos/{id}/mark_as_done",
	params(
		("id" = Uuid, Path, description = "Todo item id"),
	),
	responses(
		(status = 200, description = "Todo item marked as done successfully", body = ApiResponseTodo),
		(status = 422, description = "Todo item not exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn mark_as_done_todo_ctrl(
	State(_todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> ApiResponse<Todo> {
	let mark_as_done_usecase = mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&_todo_repo);

	let todo = mark_as_done_usecase.exec(id, true).await?;

	Ok(ApiResponseData::success_with_data(todo, StatusCode::OK))
}

#[utoipa::path(
	tag = "Todo",
	patch,
	path = "/api/todos/{id}/mark_as_undone",
	params(
		("id" = Uuid, Path, description = "Todo item id"),
	),
	responses(
		(status = 200, description = "Todo item marked as undone successfully", body = ApiResponseTodo),
		(status = 422, description = "Todo item not exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn mark_as_undone_todo_ctrl(
	State(_todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> ApiResponse<Todo> {
	let mark_as_done_usecase = mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&_todo_repo);

	let todo = mark_as_done_usecase.exec(id, false).await?;

	Ok(ApiResponseData::success_with_data(todo, StatusCode::OK))
}
