use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn render_index_ctrl() -> Result<IndexTemplate, ()> {
	Ok(IndexTemplate {})
}
