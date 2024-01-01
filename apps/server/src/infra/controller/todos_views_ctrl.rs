use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::{
	domain::{entity::todo::Todo, repository::todo_repository::DynTodoRepository},
	usecase::{
		create_todo_usecase::{self, CreateTodoParams},
		get_all_todos_usecase,
	},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
	pub num_items: i32,
	pub items: Vec<Todo>,
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
		items: todos,
	})
}

#[derive(Template)]
#[template(path = "responses/create_todo.html")]
pub struct CreateTodoResponseTemplate {
	pub num_items: i32,
	pub todo: Todo,
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
		todo,
		num_items: todo_len,
	}
}
