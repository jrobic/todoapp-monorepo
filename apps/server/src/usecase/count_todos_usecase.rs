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
		let done = match status {
			Some(status) => match status.as_str() {
				"done" => Some(&true),
				"pending" => Some(&false),
				_ => None,
			},
			None => None,
		};

		self.todo_repo.count(done).await.unwrap_or(0)
	}
}
