use axum::{extract::State, http::StatusCode};

use crate::{
	domain::{entity::todo::Todo, repository::todo_repository::DynTodoRepository},
	infra::api_response::{ApiResponse, ApiResponseData},
	usecase::create_todo_usecase,
};

#[utoipa::path(
	tag = "Todo",
	post,
	path = "/",
	request_body = Todo,
	responses(
		(status = 201, description = "Todo item created successfully", body = ApiResponseTodo),
		(status = 409, description = "Todo already exists", body = ApiResponseErrorObject),
		(status = 500, description = "Internal Server Error", body = ApiResponseErrorObject)
	)
)]
pub async fn create_todo_ctrl(State(todo_repo): State<DynTodoRepository>) -> ApiResponse<Todo> {
	let create_todo_usecase = create_todo_usecase::CreateTodoUsecase::new(&todo_repo);

	let todo = create_todo_usecase.exec("toto".to_string()).await?;

	Ok(ApiResponseData::success_with_data(
		todo,
		StatusCode::CREATED,
	))
}
