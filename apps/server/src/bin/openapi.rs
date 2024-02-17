use utoipa::OpenApi;

#[tokio::main]
async fn main() {
	let doc: utoipa::openapi::OpenApi = server::infra::api_doc::ApiDoc::openapi();

	let doc_json = doc.to_pretty_json().unwrap();

	std::fs::write("openapi.json", doc_json).unwrap();
}
