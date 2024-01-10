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

#[derive(Template, Debug, Clone, Serialize)]
#[template(path = "components/item.html")]
pub struct TodoView {
	pub id: String,
	pub description: String,
	pub done: bool,
	pub created_at: String,
	pub updated_at: String,
	pub done_at: String,
	pub need_removed_in_view: bool,
	pub need_to_update: bool,
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
			need_removed_in_view: false,
			need_to_update: false,
		}
	}
}

impl TodoView {
	pub fn set_to_be_removed_in_view(&mut self, status: String) -> &Self {
		self.need_removed_in_view = match status.as_str() {
			"done" if !self.done => true,
			"pending" if self.done => true,
			"removed" => true,
			_ => false,
		};

		self
	}

	pub fn set_to_be_update(&mut self) -> &Self {
		self.need_to_update = true;

		self
	}
}
