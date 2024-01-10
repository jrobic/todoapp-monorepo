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

#[derive(Debug, Clone, Serialize)]
pub enum TodoOperation {
	Create,
	Read,
	Update,
	Delete,
}

impl Default for TodoOperation {
	fn default() -> Self {
		Self::Read
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
	pub view_opts: TodoViewOpts,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TodoViewOpts {
	pub need_oob: bool,
	pub need_remove: bool,
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
			view_opts: TodoViewOpts::default(),
		}
	}
}

impl TodoView {
	pub fn set_view_opts(&mut self, operation: TodoOperation, status: Option<String>) -> &Self {
		self.view_opts.need_oob = !matches!(operation, TodoOperation::Read | TodoOperation::Create);

		if matches!(operation, TodoOperation::Delete) {
			self.view_opts.need_remove = true;
			return self;
		}

		self.view_opts.need_remove = match status {
			Some(status) if status == "done" && !self.done => true,
			Some(status) if status == "pending" && self.done => true,
			_ => false,
		};

		self
	}
}
