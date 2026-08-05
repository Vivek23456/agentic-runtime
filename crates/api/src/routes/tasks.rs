use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use runtime::Task;

use crate::{
    models::{CreateTaskRequest, CreateTaskResponse},
    state::AppState,
};

pub async fn create_task(
    State(state): State<AppState>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, StatusCode> {

    let task = Task::new(
        request.name,
        request.payload,
    );

    let handle = state
        .runtime
        .submit(task)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateTaskResponse {
        task_id: handle.id(),
    }))
}