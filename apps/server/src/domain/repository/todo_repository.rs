use std::sync::Arc;

use axum::async_trait;
use serde::Serialize;

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
	async fn find_many_todos(
		&self,
		args: FindManyTodoPaginatedArgs,
	) -> Result<Vec<Todo>, FindManyTodoError>;
	async fn update(&self, todo: Todo) -> Result<Todo, UpdateError>;
	async fn delete(&self, id: String) -> Result<(), DeleteError>;
	async fn count(&self, done: Option<&bool>) -> Result<i64, CountTodoError>;
}

pub type DynTodoRepository = Arc<dyn TodoRepository + Send + Sync>;

#[derive(Serialize, Debug)]
pub struct FindManyPaginatedArgs<W> {
	pub take: i64,
	pub cursor: Option<String>,
	pub filters: W,
}

#[derive(Serialize, Debug)]
pub struct FindManyPageInfo {
	pub has_next_page: bool,
	pub has_prev_page: bool,
	pub start_cursor: Option<String>,
	pub end_cursor: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FindManyPaginated<T> {
	pub data: Vec<T>,
	pub page_info: FindManyPageInfo,
}

#[derive(Serialize, Debug, Default)]
pub struct FindManyTodoArgs {
	pub done: Option<bool>,
}

impl FindManyTodoArgs {
	pub fn is_empty(&self) -> bool {
		if !self.done.is_none() {
			return false;
		}

		true
	}
}

pub type FindManyTodoPaginatedArgs = FindManyPaginatedArgs<FindManyTodoArgs>;

impl FindManyTodoPaginatedArgs {
	pub fn new(
		take: Option<i64>,
		cursor: Option<String>,
		filters: Option<FindManyTodoArgs>,
	) -> Self {
		Self {
			take: take.unwrap_or(100),
			cursor,
			filters: filters.unwrap_or_default(),
		}
	}
}
