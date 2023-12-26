use utoipa::OpenApi;

use crate::{
	domain::entity::{health::Health, todo::Todo},
	usecase::create_todo_usecase::CreateTodoParams,
};

use super::api_response::{ApiResponseErrorObject, ApiResponseObject};

#[derive(OpenApi)]
#[openapi(
	paths(
		super::controller::common_ctrl::health,
		super::controller::todo_ctrl::create_todo_ctrl,
		super::controller::todo_ctrl::get_all_todos_ctrl,
		super::controller::todo_ctrl::delete_todo_ctrl,
		super::controller::todo_ctrl::mark_as_done_todo_ctrl,
		super::controller::todo_ctrl::mark_as_undone_todo_ctrl,
	),
	components(schemas(Health, Todo, ApiResponseObject<Todo>,ApiResponseObject<Vec<Todo>>,ApiResponseErrorObject,CreateTodoParams)),
	security(),
	tags(
		(name = "Todo", description = "Todo items management API"),
	)
)]
pub struct ApiDoc;
