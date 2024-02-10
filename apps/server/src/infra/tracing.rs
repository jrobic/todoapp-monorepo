use std::time::Duration;

use opentelemetry_sdk::{
	resource::{
		EnvResourceDetector, OsResourceDetector, SdkProvidedResourceDetector,
		TelemetryResourceDetector,
	},
	Resource,
};
use opentelemetry_semantic_conventions as semconv;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn setup_tracing() {
	let fmt_layer = fmt_layer();

	let filter_layer =
		EnvFilter::try_from_default_env()
			.unwrap_or_else(|_| {
				"todoapp_server=debug,tower_http=debug,axum::rejection=trace,otel::tracing=trace,info".into()
			});

	let telemetry_layer =
		otel_layer().map(|tracer| tracing_opentelemetry::layer().with_tracer(tracer));

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(telemetry_layer)
		.init();
}

fn fmt_layer() -> tracing_subscriber::fmt::Layer<
	tracing_subscriber::layer::Layered<EnvFilter, tracing_subscriber::Registry>,
	tracing_subscriber::fmt::format::DefaultFields,
	tracing_subscriber::fmt::format::Format<
		tracing_subscriber::fmt::format::Compact,
		tracing_subscriber::fmt::time::Uptime,
	>,
> {
	tracing_subscriber::fmt::layer()
		.compact()
		.with_line_number(true)
		.with_timer(tracing_subscriber::fmt::time::uptime())
}

pub fn add_fmt_layer(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
	TraceLayer::new_for_http()
		.make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
		.on_response(trace::DefaultOnResponse::new().level(Level::INFO))
}

pub fn otel_layer() -> Option<opentelemetry_sdk::trace::Tracer> {
	let tracing_enabled: bool = std::env::var("TRACING").unwrap_or_else(|_| "0".to_string()) == "1";

	if !tracing_enabled {
		return None;
	}

	// OpenTelemetry tracing
	let exporter = opentelemetry_otlp::new_exporter().http();

	let mut resource = Resource::from_detectors(
		Duration::from_secs(0),
		vec![
			Box::new(EnvResourceDetector::new()),
			Box::new(OsResourceDetector),
			Box::new(SdkProvidedResourceDetector),
			Box::new(TelemetryResourceDetector),
		],
	);

	resource = resource.merge(&Resource::new(vec![
		semconv::resource::SERVICE_NAME.string(env!("CARGO_PKG_NAME")),
		semconv::resource::SERVICE_VERSION.string(env!("CARGO_PKG_VERSION")),
	]));

	let trace_config = opentelemetry_sdk::trace::Config::default().with_resource(resource);
	let tracer = opentelemetry_otlp::new_pipeline()
		.tracing()
		.with_exporter(exporter)
		.with_trace_config(trace_config)
		.install_batch(opentelemetry_sdk::runtime::Tokio)
		.expect("Couldn't create OTLP tracer");

	Some(tracer)
}
