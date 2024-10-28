use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{
		DynTodoRepository, FindManyTodoArgs, FindManyTodoPaginatedArgs, TodoRepository,
	},
};

pub struct GetAllTodosUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> GetAllTodosUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, status: Option<&String>) -> Result<Vec<Todo>, TodoException> {
		let done: Option<bool> = match status {
			Some(status) => match status.as_str() {
				"done" => Some(true),
				"pending" => Some(false),
				_ => None,
			},
			None => None,
		};

		let params = FindManyTodoPaginatedArgs::new(
			Some(50),
			// Some("79f4aa68-5241-4907-90a3-b86325c306e7".to_string()),
			None,
			Some(FindManyTodoArgs { done }),
		);

		match self.todo_repo.find_many_todos(params).await {
			Ok(todos) => Ok(todos),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
