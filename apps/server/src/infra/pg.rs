pub async fn create_pg_pool() -> &'static sqlx::Pool<sqlx::Postgres> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(20)
		.connect(&database_url)
		.await
		.expect("Failed to connect to Postgres");

	Box::leak(Box::new(pool))
}
