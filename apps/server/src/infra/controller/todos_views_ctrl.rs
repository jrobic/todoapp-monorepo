use askama::Template;
use askama_axum::IntoResponse;
use axum::{
	extract::{Path, State},
	http::StatusCode,
	Form,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
	domain::{entity::todo::TodoView, repository::todo_repository::DynTodoRepository},
	usecase::{
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

pub async fn render_index_ctrl(
	State(todo_repo): State<DynTodoRepository>,
) -> Result<IndexTemplate, ()> {
	let get_all_todos_usecase = get_all_todos_usecase::GetAllTodosUsecase::new(&todo_repo);

	let todos = match get_all_todos_usecase.exec().await {
		Ok(todos) => todos,
		Err(_) => return Err(()),
	};

	Ok(IndexTemplate {
		num_items: todos.clone().len() as i32,
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
}

pub async fn mark_as_done_todo_ctrl(
	State(_todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase: mark_as_done_todo_usecase::MarkAsDoneTodoUsecase<'_> =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&_todo_repo);

	let todo = mark_as_done_usecase.exec(id, true).await.unwrap();

	let todo_view: TodoView = todo.into();

	UpdateTodoTmpl { todo: todo_view }
}

pub async fn mark_as_undone_todo_ctrl(
	State(_todo_repo): State<DynTodoRepository>,
	Path(id): Path<Uuid>,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase = mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&_todo_repo);

	let todo = mark_as_done_usecase.exec(id, false).await.unwrap();

	let todo_view: TodoView = todo.into();

	UpdateTodoTmpl { todo: todo_view }
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
