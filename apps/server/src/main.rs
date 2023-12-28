mod domain;
mod infra;
mod usecase;

#[tokio::main]
async fn main() {
	infra::tracing::setup_tracing();

	let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrcis_server());
}

async fn start_main_server() {
	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

	tracing::debug!("Starting server at {}", addr);
	let app = infra::server::create_server();

	axum::serve(listener, app).await.unwrap();
}

async fn start_metrcis_server() {
	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

	tracing::debug!("Starting server at {}", addr);
	let app = infra::server::create_metrics_server();

	axum::serve(listener, app).await.unwrap();
}
