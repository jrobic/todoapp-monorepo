use std::{cmp::Reverse, sync::Mutex};

use axum::async_trait;
use chrono::{Duration, NaiveDate};
use rand::Rng;
use random_word::Lang;

use crate::domain::{
	entity::todo::Todo,
	repository::todo_repository::{
		CountTodoError, CreateTodoError, DeleteError, FindManyTodoError, FindManyTodoPaginatedArgs,
		FindTodoError, TodoRepository, UpdateError,
	},
};

pub struct TodoInMemoryRepository {
	pub todos: Mutex<Vec<Todo>>,
}

impl TodoInMemoryRepository {
	pub fn new() -> TodoInMemoryRepository {
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

impl Default for TodoInMemoryRepository {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl TodoRepository for TodoInMemoryRepository {
	async fn create_todo(&self, create_todo: Todo) -> Result<Todo, CreateTodoError> {
		let mut todos = self.todos.lock().unwrap();

		todos.push(create_todo.clone());

		Ok(create_todo)
	}

	async fn find_by_id(&self, id: String) -> Result<Todo, FindTodoError> {
		let todos = self.todos.lock().unwrap();

		let todo =
			todos.iter().find(|todo: &&Todo| todo.id == id).ok_or(FindTodoError::NotFound)?;

		Ok(todo.clone())
	}

	async fn find_many_todos(
		&self,
		args: FindManyTodoPaginatedArgs,
	) -> Result<Vec<Todo>, FindManyTodoError> {
		let mut todos: Vec<Todo> = self.todos.lock().unwrap().clone();

		if let Some(done) = args.filters.done {
			todos = todos.iter().filter(|todo| todo.done == done).cloned().collect::<Vec<Todo>>();
		}

		todos.sort_by_cached_key(|todo| Reverse(todo.created_at));

		Ok(todos)
	}

	async fn update(&self, update_todo: Todo) -> Result<Todo, UpdateError> {
		let mut todos = self.todos.lock().unwrap();

		let index = todos
			.iter()
			.position(|todo: &Todo| todo.id == update_todo.id)
			.ok_or(UpdateError::NotFound)?;

		todos[index] = update_todo.clone();

		Ok(update_todo)
	}

	async fn delete(&self, id: String) -> Result<(), DeleteError> {
		let mut todos = self.todos.lock().unwrap();

		let index = todos
			.iter()
			.position(|todo: &Todo| todo.id == id)
			.ok_or(DeleteError::NotFound)?;

		todos.remove(index);

		Ok(())
	}

	async fn count(&self, done: Option<&bool>) -> Result<i64, CountTodoError> {
		let todos: Vec<Todo> = self.todos.lock().unwrap().clone();

		let count = match done {
			Some(done) => todos.iter().filter(|todo| todo.done == *done).count(),
			None => todos.len(),
		};

		Ok(count as i64)
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
