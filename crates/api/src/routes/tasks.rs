use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use runtime::Task;
use uuid::Uuid;

use crate::{
    models::{
        CreateTaskRequest,
        CreateTaskResponse,
        TaskResponse,
    },
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

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let shared_task = state
        .runtime
        .task(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let task = shared_task.read().await;

    Ok(Json(TaskResponse {
        id: task.id,
        name: task.name.clone(),
        status: task.status.to_string(),
    }))
}

pub async fn list_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskResponse>>, StatusCode> {

    let shared_tasks = state.runtime.tasks().await;

    let mut response = Vec::with_capacity(shared_tasks.len());

    for shared in shared_tasks {
        let task = shared.read().await;

        response.push(TaskResponse {
            id: task.id,
            name: task.name.clone(),
            status: task.status.to_string(),
        });
    }

    Ok(Json(response))
}