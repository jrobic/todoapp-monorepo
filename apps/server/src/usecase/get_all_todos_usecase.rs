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
		match self.todo_repo.find_many_todos(status).await {
			Ok(todos) => Ok(todos),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
