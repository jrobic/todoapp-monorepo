use askama::Template;
use axum::extract::Query;
use random_word::Lang;
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

#[derive(Template)]
#[template(path = "components/item.html")]
pub struct ItemTemplate {
	text: String,
}

pub async fn say_hello() -> ItemTemplate {
	// sleep for 2 seconds
	// tokio::time::sleep(std::time::Duration::from_secs(1)).await;

	ItemTemplate {
		text: random_word::gen(Lang::En).to_string(),
	}
}
