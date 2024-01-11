use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DynTodoRepository, MarkAsDoneError, TodoRepository},
};

pub struct MarkAsDoneTodoUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> MarkAsDoneTodoUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, id: String, done: bool) -> Result<Todo, TodoException> {
		let todo = match self.todo_repo.mark_as_done(id, done).await {
			Ok(todo) => todo,
			Err(MarkAsDoneError::NotFound) => return Err(TodoException::NotFound),
			Err(_) => return Err(TodoException::Unknown),
		};

		Ok(todo)
	}
}
