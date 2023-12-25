use std::sync::Mutex;

use axum::async_trait;

use crate::domain::{
	entity::todo::Todo,
	repository::todo_repository::{CreateTodoError, TodoRepository},
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

		dbg!("todos len: {}", todos.len());

		Ok(todo)
	}
}
