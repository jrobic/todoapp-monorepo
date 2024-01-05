use std::sync::Mutex;

use axum::async_trait;
use random_word::Lang;
use uuid::Uuid;

use crate::domain::{
	entity::todo::Todo,
	repository::todo_repository::{
		CreateTodoError, DeleteError, FindManyTodoError, MarkAsDoneError, TodoRepository,
	},
};

pub struct TodoInMemoryRepository {
	pub todos: Mutex<Vec<Todo>>,
}

impl TodoInMemoryRepository {
	pub fn new() -> Self {
		Self {
			todos: Mutex::new(
				(1..500)
					.map(|n| Todo {
						id: Uuid::new_v4(),
						description: random_word::gen(Lang::En).to_string(),
						done: n % 3 == 0,
						created_at: chrono::Utc::now(),
						updated_at: chrono::Utc::now(),
						done_at: match n % 3 == 0 {
							true => Some(chrono::Utc::now()),
							false => None,
						},
					})
					.collect(),
			),
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

	async fn mark_as_done(&self, id: Uuid, done: bool) -> Result<Todo, MarkAsDoneError> {
		let mut todos = self.todos.lock().unwrap();

		let todo = todos
			.iter_mut()
			.find(|todo: &&mut Todo| todo.id == id)
			.ok_or(MarkAsDoneError::NotFound)?;

		todo.done = done;
		todo.done_at = if done { Some(chrono::Utc::now()) } else { None };
		todo.updated_at = chrono::Utc::now();

		Ok(todo.clone())
	}

	async fn delete(&self, id: Uuid) -> Result<(), DeleteError> {
		let mut todos = self.todos.lock().unwrap();

		let index = todos
			.iter()
			.position(|todo: &Todo| todo.id == id)
			.ok_or(DeleteError::NotFound)?;

		todos.remove(index);

		Ok(())
	}
}
