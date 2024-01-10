use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DeleteError, DynTodoRepository, TodoRepository},
};

pub struct DeleteTodoUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> DeleteTodoUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, id: Uuid) -> Result<Todo, TodoException> {
		match self.todo_repo.delete(id).await {
			Ok(todo_removed) => Ok(todo_removed),
			Err(DeleteError::NotFound) => Err(TodoException::NotFound),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
