use askama::Template;
use axum::extract::Query;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
	pub name: String,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct HelloQuery {
	pub name: Option<String>,
}

pub async fn render_hello_ctrl(query: Query<HelloQuery>) -> Result<HelloTemplate, ()> {
	let name = query.name.clone().unwrap_or("World".to_string());

	Ok(HelloTemplate { name })
}
