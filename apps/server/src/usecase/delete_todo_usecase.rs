use std::sync::Arc;

use crate::domain::{
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

	pub async fn exec(&self, id: String) -> Result<(), TodoException> {
		match self.todo_repo.delete(id).await {
			Ok(()) => Ok(()),
			Err(DeleteError::NotFound) => Err(TodoException::NotFound),
			Err(_) => Err(TodoException::Unknown),
		}
	}
}
