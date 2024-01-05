use std::collections::HashMap;

use askama::Template;
use axum::{
	extract::{Path, Query, State},
	http::HeaderMap,
	Form,
};
use serde::Deserialize;
use url::Url;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
	domain::{entity::todo::TodoView, repository::todo_repository::DynTodoRepository},
	usecase::{
		clear_all_completed_todos_usecase,
		create_todo_usecase::{self, CreateTodoParams},
		delete_todo_usecase, get_all_todos_usecase, mark_as_done_todo_usecase,
	},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
	pub num_items: i32,
	pub todos: Vec<TodoView>,
}

#[derive(Deserialize, IntoParams, Clone, Debug)]
#[into_params(parameter_in = Query)]
pub struct SearchTodosQuery {
	pub status: Option<String>,
}

pub async fn render_index_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	Query(query): Query<SearchTodosQuery>,
) -> Result<IndexTemplate, ()> {
	let get_all_todos_usecase = get_all_todos_usecase::GetAllTodosUsecase::new(&todo_repo);

	let todos = match get_all_todos_usecase.exec(query.status.clone()).await {
		Ok(todos) => todos,
		Err(_) => return Err(()),
	};

	let todo_len = todo_repo.find_many_todos(query.status.clone()).await.unwrap().len() as i32;

	Ok(IndexTemplate {
		num_items: todo_len,
		todos: todos.into_iter().map(|todo| todo.into()).collect(),
	})
}

#[derive(Template)]
#[template(path = "responses/create_todo.html")]
pub struct CreateTodoResponseTemplate {
	pub num_items: i32,
	pub todo: TodoView,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoForm {
	pub description: String,
}

pub async fn create_todo_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	Form(CreateTodoForm { description }): Form<CreateTodoForm>,
) -> CreateTodoResponseTemplate {
	let usecase: create_todo_usecase::CreateTodoUsecase<'_> =
		create_todo_usecase::CreateTodoUsecase::new(&todo_repo);

	let todo = usecase.exec(CreateTodoParams { description }).await.unwrap();

	let todo_len = todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	CreateTodoResponseTemplate {
		todo: todo.into(),
		num_items: todo_len,
	}
}

#[derive(Template)]
#[template(path = "responses/update_todo.html")]
pub struct UpdateTodoTmpl {
	pub todo: TodoView,
	pub num_items: i32,
}

pub async fn mark_as_done_todo_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
	headers: HeaderMap,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase: mark_as_done_todo_usecase::MarkAsDoneTodoUsecase<'_> =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&todo_repo);

	let todo = mark_as_done_usecase.exec(id, true).await.unwrap();

	let mut todo_view: TodoView = todo.into();

	let status = extract_status_from_header(headers);

	if let Some(status) = status.clone() {
		todo_view.set_to_be_removed_in_view(status);
	}

	let todo_len = todo_repo.find_many_todos(status).await.unwrap().len() as i32;

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

pub async fn mark_as_undone_todo_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
	headers: HeaderMap,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase = mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&todo_repo);

	let todo = mark_as_done_usecase.exec(id, false).await.unwrap();

	let mut todo_view: TodoView = todo.into();

	let status = extract_status_from_header(headers);

	if let Some(status) = status.clone() {
		todo_view.set_to_be_removed_in_view(status);
	}

	let todo_len = todo_repo.find_many_todos(status).await.unwrap().len() as i32;

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

#[derive(Template)]
#[template(path = "responses/remove_todo.html")]
pub struct RemoveTodoTmpl {
	pub num_items: i32,
}

pub async fn delete_todo_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> RemoveTodoTmpl {
	let delete_todo_usecase = delete_todo_usecase::DeleteTodoUsecase::new(&todo_repo);

	delete_todo_usecase.exec(id).await.unwrap();
	let todo_len = todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	RemoveTodoTmpl {
		num_items: todo_len,
	}
}

pub async fn clear_all_completed_todos_ctrl(
	State(todo_repo): State<DynTodoRepository>,
	headers: HeaderMap,
) -> IndexTemplate {
	let clear_all_completed_todos_usecase =
		clear_all_completed_todos_usecase::ClearAllCompletedTodosUsecase::new(&todo_repo);

	clear_all_completed_todos_usecase.exec().await.unwrap();

	let status = extract_status_from_header(headers);

	let get_all_todos_usecase = get_all_todos_usecase::GetAllTodosUsecase::new(&todo_repo);

	let todos = get_all_todos_usecase.exec(status).await.unwrap();

	let todo_len = todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	IndexTemplate {
		num_items: todo_len,
		todos: todos.into_iter().map(|todo| todo.into()).collect(),
	}
}

fn extract_status_from_header(headers: HeaderMap) -> Option<String> {
	let current_url = headers.get("hx-current-url").unwrap().to_str().unwrap();
	let hash_query: HashMap<_, _> =
		Url::parse(current_url).unwrap().query_pairs().into_owned().collect();

	hash_query.get("status").map(|status| status.to_string())
}
