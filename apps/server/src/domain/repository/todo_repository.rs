use std::sync::Arc;

use axum::async_trait;

use crate::domain::entity::todo::Todo;

#[derive(Debug)]
pub enum CreateTodoError {
	DBInternalError,
}

#[derive(Debug)]
pub enum FindTodoError {
	NotFound,
	#[allow(dead_code)]
	DBInternalError,
}

#[derive(Debug)]
pub enum FindManyTodoError {
	DBInternalError,
}

#[derive(Debug)]
pub enum UpdateError {
	NotFound,
	DBInternalError,
}

#[derive(Debug)]
pub enum DeleteError {
	NotFound,
	DBInternalError,
}

#[derive(Debug)]
pub enum CountTodoError {
	DBInternalError,
}

#[async_trait]
pub trait TodoRepository {
	async fn create_todo(&self, todo: Todo) -> Result<Todo, CreateTodoError>;
	async fn find_by_id(&self, id: String) -> Result<Todo, FindTodoError>;
	async fn find_many_todos(&self, done: Option<&bool>) -> Result<Vec<Todo>, FindManyTodoError>;
	async fn update(&self, todo: Todo) -> Result<Todo, UpdateError>;
	async fn delete(&self, id: String) -> Result<(), DeleteError>;
	async fn count(&self, done: Option<&bool>) -> Result<i64, CountTodoError>;
}

pub type DynTodoRepository = Arc<dyn TodoRepository + Send + Sync>;
