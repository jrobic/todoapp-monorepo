use std::sync::Arc;

use crate::domain::repository::todo_repository::{DynTodoRepository, TodoRepository};

pub struct CountTodosUsecase<'a> {
	pub todo_repo: &'a Arc<dyn TodoRepository + Send + Sync>,
}

impl<'a> CountTodosUsecase<'a> {
	pub fn new(todo_repo: &'a DynTodoRepository) -> Self {
		Self { todo_repo }
	}

	pub async fn exec(&self, status: Option<&String>) -> i64 {
		self.todo_repo.count(status).await.unwrap_or(0)
	}
}
