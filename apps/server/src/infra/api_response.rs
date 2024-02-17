use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};

use regex::Regex;

use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::todo::Todo;

pub enum ApiResponseType {
	SuccessWithData,
	#[allow(dead_code)]
	StatusCodeOnly,
	#[allow(dead_code)]
	Error,
}

impl Default for ApiResponseType {
	fn default() -> Self {
		Self::SuccessWithData
	}
}

#[derive(ToSchema)]
pub enum ApiResponseData<T: Serialize, I: Serialize> {
	Data {
		data: T,
		informations: Option<I>,
		status: StatusCode,
	},
	#[allow(dead_code)]
	Error(ApiResponseError),
	#[allow(dead_code)]
	StatusCode(StatusCode),
}

impl<T, I> ApiResponseData<T, I>
where
	T: Serialize + 'static,
	I: Serialize + 'static,
{
	pub fn success_with_data(data: T, informations: Option<I>, status: StatusCode) -> Self {
		Self::Data {
			data,
			informations,
			status,
		}
	}

	#[allow(dead_code)]
	pub fn status_code(status: StatusCode) -> Self {
		Self::StatusCode(status)
	}
	#[allow(dead_code)]
	pub fn error(message: &'static str) -> Self {
		let err = anyhow::format_err!(message);

		Self::Error(err.into())
	}
}

impl<T, I> IntoResponse for ApiResponseData<T, I>
where
	T: Serialize,
	I: Serialize,
{
	fn into_response(self) -> Response {
		match self {
			ApiResponseData::Data {
				data,
				informations,
				status,
			} => (
				status,
				Json(ApiResponseObject::<T, I> {
					status: status.to_string(),
					informations,
					data,
				}),
			)
				.into_response(),
			ApiResponseData::Error(error) => error.into_response(),
			ApiResponseData::StatusCode(status) => status.into_response(),
		}
	}
}
#[derive(Serialize, ToSchema)]
pub struct ListInformations {
	pub total: i64,
}
#[derive(Serialize, ToSchema)]
pub struct TodoParams {}

#[derive(Serialize, ToSchema)]
// it's not possible to use a generic type as a field in a struct with utoipa
// it's not ideal but we can use aliases to workaround this limitation
#[aliases(ApiResponseTodo = ApiResponseObject<Todo, TodoParams>, ApiResponseListTodos = ApiResponseObject<Vec<Todo>, ListInformations>)]
pub struct ApiResponseObject<T, I>
where
	T: Serialize,
	I: Serialize,
{
	status: String,
	informations: Option<I>,
	data: T,
}

pub type ApiResponse<T, I> = Result<ApiResponseData<T, I>, ApiResponseError>;

pub struct ApiResponseError(anyhow::Error);

#[derive(Serialize, ToSchema)]
pub struct ApiResponseErrorObject {
	status: String,
	error: String,
}

impl<E> From<E> for ApiResponseError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

impl IntoResponse for ApiResponseError {
	fn into_response(self) -> Response {
		let msg = self.0.to_string();

		// parse msg to get status code and error message
		// "[409] Todo already exists"
		let re = Regex::new(r"^\[(\d+)\] (.+)$").unwrap();

		match re.captures(&msg) {
			Some(captures) => {
				let status_code = match captures.get(1).unwrap().as_str().parse::<u16>() {
					Ok(status_code) => StatusCode::from_u16(status_code)
						.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
					Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
				};

				let error = captures.get(2).unwrap().as_str();

				(
					status_code,
					Json(ApiResponseErrorObject {
						status: status_code.to_string(),
						error: error.to_string(),
					}),
				)
			},
			None => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ApiResponseErrorObject {
					status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
					error: msg,
				}),
			),
		}
		.into_response()
	}
}
