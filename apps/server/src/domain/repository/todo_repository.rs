use std::sync::Arc;

use axum::async_trait;

use crate::domain::entity::todo::Todo;

#[derive(Debug)]
pub enum CreateTodoError {
	AlreadyExists,
	#[allow(dead_code)]
	DBInternalError,
}

#[derive(Debug)]
pub enum FindTodoError {
	#[allow(dead_code)]
	NotFound,
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

#[derive(Debug)]
pub enum CountTodoError {
	#[allow(dead_code)]
	DBInternalError,
}

#[async_trait]
pub trait TodoRepository {
	async fn create_todo(&self, description: String) -> Result<Todo, CreateTodoError>;
	async fn find_many_todos(
		&self,
		status: Option<&String>,
	) -> Result<Vec<Todo>, FindManyTodoError>;
	async fn mark_as_done(&self, id: String, done: bool) -> Result<Todo, MarkAsDoneError>;
	async fn delete(&self, id: String) -> Result<Todo, DeleteError>;
	async fn delete_done_todos(&self) -> Result<(), DeleteError>;
	async fn count(&self, status: Option<&String>) -> Result<i64, CountTodoError>;
}

pub type DynTodoRepository = Arc<dyn TodoRepository + Send + Sync>;
