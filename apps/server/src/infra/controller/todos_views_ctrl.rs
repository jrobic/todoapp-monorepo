use std::{collections::HashMap, convert::Infallible, time::Duration};

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
	extract::{Path, Query, State},
	http::{HeaderMap, StatusCode},
	response::{
		sse::{Event, KeepAlive},
		Sse,
	},
	Form,
};
use serde::{Deserialize, Serialize};
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt as _};
use url::Url;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
	domain::entity::todo::TodoView,
	infra::server::AppState,
	usecase::{
		clear_all_completed_todos_usecase,
		create_todo_usecase::{self, CreateTodoParams},
		delete_todo_usecase, get_all_todos_usecase, mark_as_done_todo_usecase,
	},
};

#[derive(Template)]
#[template(path = "views/index.html")]
pub struct IndexTemplate {
	pub num_items: i32,
	pub todos: Vec<TodoView>,
}

#[derive(Template)]
#[template(path = "responses/update_todo.html")]
pub struct UpdateTodoTmpl {
	pub todo: TodoView,
	pub num_items: i32,
}

#[derive(Template)]
#[template(path = "views/stream.html")]
pub struct StreamTmpl {}

#[derive(Deserialize, IntoParams, Clone, Debug)]
#[into_params(parameter_in = Query)]
pub struct SearchTodosQuery {
	pub status: Option<String>,
}

pub async fn render_index_ctrl(
	State(app_state): State<AppState>,
	Query(query): Query<SearchTodosQuery>,
) -> Result<IndexTemplate, ()> {
	let get_all_todos_usecase =
		get_all_todos_usecase::GetAllTodosUsecase::new(&app_state.todo_repo);

	let todos = match get_all_todos_usecase.exec(query.status.clone()).await {
		Ok(todos) => todos,
		Err(_) => return Err(()),
	};

	let todo_len =
		app_state.todo_repo.find_many_todos(query.status.clone()).await.unwrap().len() as i32;

	Ok(IndexTemplate {
		num_items: todo_len,
		todos: todos.into_iter().map(|todo| todo.into()).collect(),
	})
}

#[derive(Template)]
#[template(path = "responses/list_todos.html")]
pub struct ListTodosTmpl {
	pub todos: Vec<TodoView>,
	pub num_items: i32,
}

pub async fn list_todos_ctrl(
	State(app_state): State<AppState>,
	Query(query): Query<SearchTodosQuery>,
	headers: HeaderMap,
) -> ListTodosTmpl {
	let get_all_todos_usecase =
		get_all_todos_usecase::GetAllTodosUsecase::new(&app_state.todo_repo);

	let status = extract_status_from_header(headers);

	let todos = get_all_todos_usecase.exec(query.status.clone().or(status)).await.unwrap();

	let todo_len =
		app_state.todo_repo.find_many_todos(query.status.clone()).await.unwrap().len() as i32;

	ListTodosTmpl {
		todos: todos.into_iter().map(|todo| todo.into()).collect(),
		num_items: todo_len,
	}
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoForm {
	pub description: String,
}

pub async fn test_sse(State(app_state): State<AppState>) -> impl IntoResponse {
	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::Create,
		todos: vec![TodoView {
			id: Uuid::new_v4().to_string(),
			description: "test".to_string(),
			created_at: "".to_string(),
			updated_at: "".to_string(),
			done: false,
			done_at: "".to_string(),
			need_removed_in_view: false,
			need_to_update: false,
		}],
		num_items: 32,
	});

	StatusCode::OK
}

pub async fn create_todo_ctrl(
	State(app_state): State<AppState>,
	Form(CreateTodoForm { description }): Form<CreateTodoForm>,
	// Extension(tx): Extension<TodosStream>,
) -> UpdateTodoTmpl {
	let usecase: create_todo_usecase::CreateTodoUsecase<'_> =
		create_todo_usecase::CreateTodoUsecase::new(&app_state.todo_repo);

	let todo = usecase.exec(CreateTodoParams { description }).await.unwrap();

	let todo_len = app_state.todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	let todo_view: TodoView = todo.into();

	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::Create,
		todos: vec![todo_view.clone()],
		num_items: todo_len,
	});

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

pub async fn mark_as_done_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<Uuid>,
	headers: HeaderMap,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase: mark_as_done_todo_usecase::MarkAsDoneTodoUsecase<'_> =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&app_state.todo_repo);

	let todo = mark_as_done_usecase.exec(id, true).await.unwrap();

	let mut todo_view: TodoView = todo.into();
	todo_view.set_to_be_update();

	let status = extract_status_from_header(headers);

	if let Some(status) = status.clone() {
		todo_view.set_to_be_removed_in_view(status);
	}

	let todo_len = app_state.todo_repo.find_many_todos(status).await.unwrap().len() as i32;

	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::MarkAsDone,
		todos: vec![todo_view.clone()],
		num_items: todo_len,
	});

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

pub async fn mark_as_undone_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<Uuid>,
	headers: HeaderMap,
) -> UpdateTodoTmpl {
	let mark_as_done_usecase =
		mark_as_done_todo_usecase::MarkAsDoneTodoUsecase::new(&app_state.todo_repo);

	let todo = mark_as_done_usecase.exec(id, false).await.unwrap();

	let mut todo_view: TodoView = todo.into();
	todo_view.set_to_be_update();

	let status = extract_status_from_header(headers);

	if let Some(status) = status.clone() {
		todo_view.set_to_be_removed_in_view(status);
	}

	let todo_len = app_state.todo_repo.find_many_todos(status).await.unwrap().len() as i32;

	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::MarkAsUndone,
		todos: vec![todo_view.clone()],
		num_items: todo_len,
	});

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

pub async fn delete_todo_ctrl(
	State(app_state): State<AppState>,
	Path(id): Path<Uuid>,
) -> UpdateTodoTmpl {
	let delete_todo_usecase = delete_todo_usecase::DeleteTodoUsecase::new(&app_state.todo_repo);

	let todo = delete_todo_usecase.exec(id).await.unwrap();
	let todo_len = app_state.todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	let mut todo_view: TodoView = todo.into();
	todo_view.set_to_be_removed_in_view("removed".to_string());
	todo_view.set_to_be_update();

	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::Remove,
		todos: vec![todo_view.clone()],
		num_items: todo_len,
	});

	UpdateTodoTmpl {
		todo: todo_view,
		num_items: todo_len,
	}
}

pub async fn clear_all_completed_todos_ctrl(
	State(app_state): State<AppState>,
	headers: HeaderMap,
) -> IndexTemplate {
	let clear_all_completed_todos_usecase =
		clear_all_completed_todos_usecase::ClearAllCompletedTodosUsecase::new(&app_state.todo_repo);

	clear_all_completed_todos_usecase.exec().await.unwrap();

	let status = extract_status_from_header(headers);

	let get_all_todos_usecase =
		get_all_todos_usecase::GetAllTodosUsecase::new(&app_state.todo_repo);

	let todos = get_all_todos_usecase.exec(status).await.unwrap();

	let todo_len = app_state.todo_repo.find_many_todos(None).await.unwrap().len() as i32;

	app_state.broadcast_update_to_view(TodoUpdate {
		kind: MutationKind::ClearAllCompleted,
		todos: vec![],
		num_items: todo_len,
	});

	IndexTemplate {
		num_items: todo_len,
		todos: todos
			.into_iter()
			.map(|todo| {
				let mut todo_view: TodoView = todo.into();
				todo_view.set_to_be_update();
				todo_view
			})
			.collect(),
	}
}

fn extract_status_from_header(headers: HeaderMap) -> Option<String> {
	let current_url = headers.get("hx-current-url").unwrap().to_str().unwrap();
	let hash_query: HashMap<_, _> =
		Url::parse(current_url).unwrap().query_pairs().into_owned().collect();

	hash_query.get("status").map(|status| status.to_string())
}

#[derive(Debug, Clone, Serialize)]
pub enum MutationKind {
	Create,
	MarkAsDone,
	MarkAsUndone,
	Remove,
	ClearAllCompleted,
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoUpdate {
	pub kind: MutationKind,
	pub todos: Vec<TodoView>,
	pub num_items: i32,
}

impl TodoUpdate {
	pub fn new(kind: MutationKind, todos: Vec<TodoView>, num_items: i32) -> Self {
		Self {
			kind,
			todos,
			num_items,
		}
	}
}

pub async fn todos_stream(
	State(app_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
	let rx = app_state.tx.subscribe();

	let stream = BroadcastStream::new(rx);

	Sse::new(
		stream
			.map(
				|msg: Result<
					TodoUpdate,
					tokio_stream::wrappers::errors::BroadcastStreamRecvError,
				>| {
					let msg = msg.unwrap();

					let body = UpdateTodoTmpl {
						todo: msg.todos[0].clone(),
						num_items: msg.num_items,
					};

					Event::default().event("update_todo_view").data(body.render().unwrap())
				},
			)
			.map(Ok),
	)
	.keep_alive(KeepAlive::new().interval(Duration::from_secs(600)).text("keep-alive-text"))
}

pub async fn stream() -> impl IntoResponse {
	StreamTmpl {}
}
