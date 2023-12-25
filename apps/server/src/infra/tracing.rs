use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn setup_tracing() {
	let fmt_layer = tracing_subscriber::fmt::layer().compact();

	let filter_layer = EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| "todoapp_server=debug,tower_http=debug,axum::rejection=trace".into());

	tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
}

pub fn add_tracing_layer(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
	TraceLayer::new_for_http()
		.make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
		.on_response(trace::DefaultOnResponse::new().level(Level::INFO))
}
