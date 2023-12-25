use ::serde::Serialize;

use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Default, Debug)]
pub struct Health {
	pub status: String,
	pub version: String,
	pub uptime: u64,
}
