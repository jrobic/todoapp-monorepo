use std::{cmp::Reverse, sync::Mutex};

use axum::async_trait;
use chrono::{Duration, NaiveDate};
use rand::Rng;
use random_word::Lang;

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
				(1..501)
					.map(|n| {
						let created_date = random_date_in_range(
							&mut rand::thread_rng(),
							NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
							NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
						);

						let mut todo = Todo::new(random_word::gen(Lang::En).to_string());

						todo.done = n % 3 == 0;
						todo.created_at = created_date;
						todo.updated_at = created_date;
						todo.done_at = match n % 3 == 0 {
							true => Some(chrono::Utc::now()),
							false => None,
						};

						todo
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
		status: Option<&String>,
	) -> Result<Vec<Todo>, FindManyTodoError> {
		let mut todos = self.todos.lock().unwrap().clone();

		if let Some(status) = status {
			todos = todos
				.iter()
				.filter(|todo| match status {
					s if s == "done" => todo.done,
					s if s == "pending" => !todo.done,
					_ => true,
				})
				.cloned()
				.collect::<Vec<Todo>>();
		}

		todos.sort_by_cached_key(|todo| Reverse(todo.created_at));

		Ok(todos)
	}

	async fn mark_as_done(&self, id: String, done: bool) -> Result<Todo, MarkAsDoneError> {
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

	async fn delete(&self, id: String) -> Result<Todo, DeleteError> {
		let mut todos = self.todos.lock().unwrap();

		let index = todos
			.iter()
			.position(|todo: &Todo| todo.id == id)
			.ok_or(DeleteError::NotFound)?;

		Ok(todos.remove(index))
	}

	async fn delete_done_todos(&self) -> Result<(), DeleteError> {
		let mut todos = self.todos.lock().unwrap();

		todos.retain(|todo| !todo.done);

		Ok(())
	}
}

fn random_date_in_range(
	rng: &mut rand::rngs::ThreadRng,
	start: NaiveDate,
	end: NaiveDate,
) -> chrono::DateTime<chrono::Utc> {
	let days_in_range = (end - start).num_days();
	let random_days: i64 = rng.gen_range(0..days_in_range);

	chrono::Utc::now() - Duration::days(random_days)
}
