use std::sync::Arc;

use axum::async_trait;

use crate::domain::entity::todo::Todo;

#[derive(Debug)]
pub enum CreateTodoError {
	AlreadyExists,
	DBInternalError,
}

#[async_trait]
pub trait TodoRepository {
	async fn create_todo(&self, description: String) -> Result<Todo, CreateTodoError>;
}

pub type DynTodoRepository = Arc<dyn TodoRepository + Send + Sync>;
