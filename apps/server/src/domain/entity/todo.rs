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
	pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
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
			deleted_at: None,
			done_at: None,
		}
	}
}
