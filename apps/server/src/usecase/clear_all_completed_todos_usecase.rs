use std::sync::Arc;

use crate::domain::{
	exception::TodoException,
	repository::todo_repository::{DeleteError, DynTodoRepository, TodoRepository},
};

pub struct ClearAllCompletedTodosUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> ClearAllCompletedTodosUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self) -> Result<(), TodoException> {
		match self.todo_repo.delete_done_todos().await {
			Ok(_) => Ok(()),
			Err(DeleteError::NotFound) => Ok(()),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
