use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum TodoException {
	#[error("[409] Todo already exists")]
	AlreadyExists,
	#[error("[500] Unknown error")]
	Unknown,
}
