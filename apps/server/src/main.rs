mod domain;
mod infra;
// mod otel;
mod usecase;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	infra::tracing::setup_tracing();

	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

	tracing::info!("Starting server at {}", addr);

	let app = infra::server::create_server();

	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}

async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	tracing::warn!("signal received, starting graceful shutdown");
	// opentelemetry::global::shutdown_tracer_provider(); //FIXME: not working
}
