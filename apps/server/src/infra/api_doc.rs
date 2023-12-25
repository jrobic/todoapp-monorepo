use utoipa::OpenApi;

use crate::domain::entity::{health::Health, todo::Todo};

use super::api_response::{ApiResponseErrorObject, ApiResponseObject};

#[derive(OpenApi)]
#[openapi(
	paths(
		super::controller::common_ctrl::health,
		super::controller::todo_ctrl::create_todo_ctrl,
	),
	components(schemas(Health, Todo, ApiResponseObject<Todo>,ApiResponseErrorObject)),
	security(),
	tags(
		(name = "Todo", description = "Todo items management API"),
	)
)]
pub struct ApiDoc;
