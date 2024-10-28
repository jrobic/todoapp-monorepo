use std::fmt::Display;

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Default, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
	pub id: String,
	pub description: String,
	pub done: bool,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
	pub done_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Todo {
	pub fn new(description: String) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			description,
			done: false,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
			done_at: None,
		}
	}


	pub fn mark_as_done(&mut self, done: bool) -> &mut Self {
		self.done = done;
		self.done_at = if done { Some(chrono::Utc::now()) } else { None };
		self.updated_at = chrono::Utc::now();

		self
	}
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TodoOperation {
	Create,
	Read,
	#[allow(dead_code)]
	Update,
	MarkAsDone,
	MarkAsUndone,
	Delete,
}

impl Display for TodoOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoView {
	pub id: String,
	pub description: String,
	pub done: bool,
	pub created_at: String,
	pub updated_at: String,
	pub done_at: String,
	pub kind: String,
	pub can: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TodoCan {
	Read,
	Write,
}

impl Display for TodoCan {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl TodoView {
	pub fn new(todo: Todo, kind: TodoOperation, can: TodoCan) -> Self {
		Self {
			id: todo.id,
			description: todo.description,
			done: todo.done,
			created_at: todo.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			updated_at: todo.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			done_at: todo
				.done_at
				.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
				.unwrap_or_default(),
			kind: kind.to_string(),
			can: can.to_string().to_uppercase(),
		}
	}
}
