use std::sync::Arc;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DynTodoRepository, TodoRepository},
};

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct CreateTodoParams {
	#[schema(example = "Buy milk")]
	pub description: String,
}

pub struct CreateTodoUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> CreateTodoUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, params: CreateTodoParams) -> Result<Todo, TodoException> {
		let todo = Todo::new(params.description);

		let new_todo = match self.todo_repo.create_todo(todo).await {
			Ok(todo) => todo,
			Err(_) => return Err(TodoException::Unknown),
		};

		Ok(new_todo)
	}
}
