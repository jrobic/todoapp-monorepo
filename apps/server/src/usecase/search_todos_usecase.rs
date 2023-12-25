use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DynTodoRepository, TodoRepository},
};

pub struct SearchTodosUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> SearchTodosUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, search_term: impl ToString) -> Result<Vec<Todo>, TodoException> {
		match self.todo_repo.find_many_todos(Some(search_term.to_string())).await {
			Ok(todos) => Ok(todos),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
