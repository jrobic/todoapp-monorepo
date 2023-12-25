mod domain;
mod infra;
mod usecase;

#[tokio::main]
async fn main() {
	infra::tracing::setup_tracing();

	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

	tracing::info!("Starting server at {}", addr);

	let app = infra::server::create_server();

	axum::serve(listener, app).await.unwrap();
}
