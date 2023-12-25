use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{CreateTodoError, DynTodoRepository, TodoRepository},
};

pub struct CreateTodoUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> CreateTodoUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, description: String) -> Result<Todo, TodoException> {
		let todo = match self.todo_repo.create_todo(description).await {
			Ok(todo) => todo,
			Err(CreateTodoError::AlreadyExists) => return Err(TodoException::AlreadyExists),
			Err(_) => return Err(TodoException::Unknown),
		};

		Ok(todo)
	}
}
