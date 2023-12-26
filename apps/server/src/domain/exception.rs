use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum TodoException {
	#[error("[409] Todo already exists")]
	AlreadyExists,
	#[error("[422] Todo not exists")]
	NotFound,
	#[error("[500] Unknown error")]
	Unknown,
}
