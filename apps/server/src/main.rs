mod domain;
mod infrastructure;
mod usecase;

#[tokio::main]
async fn main() {
	infrastructure::tracing::setup_tracing();

	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

	tracing::info!("Starting server at {}", addr);

	let app = infrastructure::server::create_server();

	axum::serve(listener, app).await.unwrap();
}
