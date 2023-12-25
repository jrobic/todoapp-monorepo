use axum::{
	extract::{Query, State},
	http::StatusCode,
	Json,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
	domain::{entity::todo::Todo, repository::todo_repository::DynTodoRepository},
	infra::api_response::{ApiResponse, ApiResponseData},
	usecase::{
		create_todo_usecase::{self, CreateTodoParams},
		get_all_todos_usecase, search_todos_usecase,
	},
};

#[utoipa::path(
	tag = "Todo",
	post,
	path = "/todos",
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
	path = "/todos",
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
			dbg!("searching");
			let search_todos_usecase = search_todos_usecase::SearchTodosUsecase::new(&todo_repo);
			let todos = search_todos_usecase.exec(st).await?;

			Ok(ApiResponseData::success_with_data(todos, StatusCode::OK))
		},
		None => {
			dbg!("all");
			let get_all_todos_usecase = get_all_todos_usecase::GetAllTodosUsecase::new(&todo_repo);

			let todos = get_all_todos_usecase.exec().await?;

			Ok(ApiResponseData::success_with_data(todos, StatusCode::OK))
		},
	}
}
