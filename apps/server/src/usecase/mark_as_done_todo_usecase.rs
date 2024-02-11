use std::sync::Arc;

use crate::domain::{
	entity::todo::Todo,
	exception::TodoException,
	repository::todo_repository::{DynTodoRepository, TodoRepository, UpdateError},
};

pub struct MarkAsDoneTodoUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> MarkAsDoneTodoUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, id: String, done: bool) -> Result<Todo, TodoException> {
		let mut todo = self.todo_repo.find_by_id(id).await.unwrap();

		todo = todo.mark_as_done(done).to_owned();

		match self.todo_repo.update(todo.clone()).await {
			Ok(todo) => todo,
			Err(UpdateError::NotFound) => return Err(TodoException::NotFound),
			Err(_) => return Err(TodoException::Unknown),
		};

		Ok(todo)
	}
}
