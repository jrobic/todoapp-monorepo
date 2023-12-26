use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;

use crate::domain::entity::todo::Todo;

#[derive(Debug)]
pub enum CreateTodoError {
	AlreadyExists,
	#[allow(dead_code)]
	DBInternalError,
}

#[derive(Debug)]
pub enum FindManyTodoError {
	#[allow(dead_code)]
	DBInternalError,
}

#[derive(Debug)]
pub enum MarkAsDoneError {
	NotFound,
	#[allow(dead_code)]
	DBInternalError,
}

#[derive(Debug)]
pub enum DeleteError {
	NotFound,
	#[allow(dead_code)]
	DBInternalError,
}

#[async_trait]
pub trait TodoRepository {
	async fn create_todo(&self, description: String) -> Result<Todo, CreateTodoError>;
	async fn find_many_todos(
		&self,
		search_term: Option<String>,
	) -> Result<Vec<Todo>, FindManyTodoError>;
	async fn mark_as_done(&self, id: Uuid, done: bool) -> Result<Todo, MarkAsDoneError>;
	async fn delete(&self, id: Uuid) -> Result<(), DeleteError>;
}

pub type DynTodoRepository = Arc<dyn TodoRepository + Send + Sync>;
