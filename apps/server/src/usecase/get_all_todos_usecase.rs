use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DynTodoRepository, TodoRepository},
};

pub struct GetAllTodosUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> GetAllTodosUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, status: Option<&String>) -> Result<Vec<Todo>, TodoException> {
		let done: Option<&bool> = match status {
			Some(status) => match status.as_str() {
				"done" => Some(&true),
				"pending" => Some(&false),
				_ => None,
			},
			None => None,
		};

		match self.todo_repo.find_many_todos(done).await {
			Ok(todos) => Ok(todos),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
