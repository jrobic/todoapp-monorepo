use server::{domain::entity::todo::Todo, infra::pg::create_pg_pool};

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let pool = create_pg_pool().await;

	sqlx::migrate!("./migrations")
		.run(pool)
		.await
		.expect("Failed to migrate the database");

	println!("Migrated the database");

	println!("-----------------------");
	truncate_todos(pool).await.expect("Failed to truncate todos");
	println!("Truncated todos");

	let tasks: Vec<_> = (0..5000).map(|_| create_todo(pool)).collect();

	let todos: Vec<Todo> = futures::future::join_all(tasks)
		.await
		.into_iter()
		.filter_map(Result::ok)
		.collect();

	println!("Created {} todos", todos.len());
}

async fn truncate_todos(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
	sqlx::query("TRUNCATE TABLE todos").execute(pool).await.map(|_| ())
}

async fn create_todo(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Todo, sqlx::Error> {
	let todo = Todo::new(random_word::gen(random_word::Lang::En).to_string());

	sqlx::query_as::<_, Todo>("INSERT INTO todos (id, description, done, created_at, updated_at, done_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
		.bind(todo.id)
		.bind(todo.description)
		.bind(todo.done)
		.bind(todo.created_at)
		.bind(todo.updated_at)
		.bind(todo.done_at)
		.fetch_one(pool)
		.await
		.map_err(|err| {
			tracing::error!("Error creating todo: {:?}", err);
			err
		})
}
