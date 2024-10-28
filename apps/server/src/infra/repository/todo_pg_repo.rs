use axum::async_trait;
use sqlx::{prelude::FromRow, Execute};
use tracing::instrument;

use crate::domain::{
	entity::todo::Todo,
	repository::todo_repository::{
		CountTodoError, CreateTodoError, DeleteError, FindManyTodoError, FindManyTodoPaginatedArgs,
		FindTodoError, TodoRepository, UpdateError,
	},
};

#[derive(Debug)]
pub struct TodoPgRepository<'a> {
	pool: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> TodoPgRepository<'a> {
	pub fn new(pool: &'a sqlx::Pool<sqlx::Postgres>) -> Self {
		Self { pool }
	}
}

#[derive(FromRow)]
struct TodosCount {
	count: i64,
}

#[async_trait]
impl<'a> TodoRepository for TodoPgRepository<'a> {
	#[instrument(name = "sqlx::create_todo")]
	async fn create_todo(&self, todo: Todo) -> Result<Todo, CreateTodoError> {
		sqlx::query_as::<_, Todo>("INSERT INTO todos (id, description, done, created_at, updated_at, done_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
			.bind(todo.id)
			.bind(todo.description)
			.bind(todo.done)
			.bind(todo.created_at)
			.bind(todo.updated_at)
			.bind(todo.done_at)
			.fetch_one(self.pool)
			.await
			.map_err(|err| {
				tracing::error!("Error creating todo: {:?}", err);
				CreateTodoError::DBInternalError
			})
	}

	#[instrument(name = "sqlx::find_by_id")]
	async fn find_by_id(&self, id: String) -> Result<Todo, FindTodoError> {
		sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
			.bind(id)
			.fetch_one(self.pool)
			.await
			.map_err(|err| {
				tracing::error!("Error finding todo: {:?}", err);
				FindTodoError::NotFound
			})
	}

	#[instrument(name = "sqlx::find_many_todos")]
	async fn find_many_todos(
		&self,
		args: FindManyTodoPaginatedArgs,
	) -> Result<Vec<Todo>, FindManyTodoError> {
		let mut query_builder = sqlx::query_builder::QueryBuilder::new("SELECT * FROM todos");
		let mut have_where_clause = false;

		if !args.filters.is_empty() || args.cursor.is_some() {
			query_builder.push(" WHERE");
		}

		if let Some(cursor) = args.cursor {
			query_builder.push(" id > ");
			query_builder.push_bind(cursor);
			have_where_clause = true;
		}

		if let Some(done) = args.filters.done {
			if have_where_clause {
				query_builder.push(" AND");
			}

			query_builder.push(" done = ");
			query_builder.push_bind(done);
			// have_where_clause = true;
		}

		query_builder.push(" ORDER BY created_at DESC");

		query_builder.push(" LIMIT ");
		query_builder.push_bind(args.take);

		query_builder.push(";");

		let query = query_builder.build_query_as();

		dbg!(query.sql().to_string());

		query.fetch_all(self.pool).await.map_err(|err| {
			tracing::error!("Error finding todos: {:?}", err);
			FindManyTodoError::DBInternalError
		})
	}

	// async fn find_many_todos(
	// 	&self,
	// 	args: FindManyTodoPaginatedArgs,
	// ) -> Result<Vec<Todo>, FindManyTodoError> {
	// 	match args.filters.done {
	// 		Some(done) => {
	// 			sqlx::query_as::<_, Todo>(
	// 				"SELECT * FROM todos WHERE done = $1 ORDER BY created_at DESC",
	// 			)
	// 			.bind(done)
	// 			.fetch_all(self.pool)
	// 			.await
	// 		},
	// 		None => {
	// 			sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
	// 				.fetch_all(self.pool)
	// 				.await
	// 		},
	// 	}
	// 	.map_err(|err| {
	// 		tracing::error!("Error finding todos: {:?}", err);
	// 		FindManyTodoError::DBInternalError
	// 	})
	// }

	#[instrument(name = "sqlx::update_todo")]
	async fn update(&self, update_todo: Todo) -> Result<Todo, UpdateError> {
		sqlx::query_as::<_, Todo>("UPDATE todos SET description = $1, done = $2, updated_at = $3, done_at = $4 WHERE id = $5 RETURNING *")
			.bind(update_todo.description)
			.bind(update_todo.done)
			.bind(update_todo.updated_at)
			.bind(update_todo.done_at)
			.bind(update_todo.id)
			.fetch_one(self.pool)
			.await
			.map_err(|err| {
				tracing::error!("Error updating todo: {:?}", err);
				UpdateError::DBInternalError
			})
	}

	#[instrument(name = "sqlx::delete_todo")]
	async fn delete(&self, id: String) -> Result<(), DeleteError> {
		sqlx::query("DELETE FROM todos WHERE id = $1")
			.bind(id)
			.execute(self.pool)
			.await
			.map_err(|err| {
				tracing::error!("Error deleting todo: {:?}", err);
				DeleteError::DBInternalError
			})
			.map(|_| ())
	}

	#[instrument(name = "sqlx::count_todos")]
	async fn count(&self, done: Option<&bool>) -> Result<i64, CountTodoError> {
		match done {
			Some(done) => {
				sqlx::query_as::<_, TodosCount>("SELECT COUNT(*) FROM todos WHERE done = $1")
					.bind(done)
					.fetch_one(self.pool)
					.await
			},
			None => {
				sqlx::query_as::<_, TodosCount>("SELECT COUNT(*) FROM todos")
					.fetch_one(self.pool)
					.await
			},
		}
		.map_err(|err| {
			tracing::error!("Error counting todos: {:?}", err);
			CountTodoError::DBInternalError
		})
		.map(|count| count.count)
	}
}
