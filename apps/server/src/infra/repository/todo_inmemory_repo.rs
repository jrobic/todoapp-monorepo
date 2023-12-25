use std::sync::Mutex;

use axum::async_trait;

use crate::domain::{
	entity::todo::Todo,
	repository::todo_repository::{CreateTodoError, FindManyTodoError, TodoRepository},
};

pub struct TodoInMemoryRepository {
	pub todos: Mutex<Vec<Todo>>,
}

impl TodoInMemoryRepository {
	pub fn new() -> Self {
		Self {
			todos: Mutex::new(vec![]),
		}
	}
}

#[async_trait]
impl TodoRepository for TodoInMemoryRepository {
	async fn create_todo(&self, description: String) -> Result<Todo, CreateTodoError> {
		let mut todos = self.todos.lock().unwrap();

		if todos.iter().any(|todo| todo.description == description) {
			return Err(CreateTodoError::AlreadyExists);
		}

		let todo = Todo::new(description);

		todos.push(todo.clone());

		Ok(todo)
	}

	async fn find_many_todos(
		&self,
		search_term: Option<String>,
	) -> Result<Vec<Todo>, FindManyTodoError> {
		let todos = self.todos.lock().unwrap();

		if let Some(search_term) = search_term {
			let todos = todos
				.iter()
				.filter(|todo| {
					todo.description.to_lowercase().contains(&search_term.to_lowercase())
				})
				.cloned()
				.collect::<Vec<Todo>>();

			return Ok(todos);
		}

		Ok(todos.clone())
	}
}
