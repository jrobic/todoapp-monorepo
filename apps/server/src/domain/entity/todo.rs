use askama::Template;
use serde::Serialize;

use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
	pub id: Uuid,
	pub description: String,
	pub done: bool,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
	pub done_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "components/item.html")]
pub struct TodoView {
	pub id: String,
	pub description: String,
	pub done: bool,
	pub created_at: String,
	pub updated_at: String,
	pub done_at: String,
}

impl From<Todo> for TodoView {
	fn from(todo: Todo) -> Self {
		Self {
			id: todo.id.to_string(),
			description: todo.description,
			done: todo.done,
			created_at: todo.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			updated_at: todo.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
			done_at: todo
				.done_at
				.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
				.unwrap_or_default(),
		}
	}
}

impl Todo {
	pub fn new(description: String) -> Self {
		Self {
			id: Uuid::new_v4(),
			description,
			done: false,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
			done_at: None,
		}
	}
}
