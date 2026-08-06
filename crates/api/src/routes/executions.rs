use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    models::{
        CreateExecutionRequest,
        CreateExecutionResponse,
        ExecutionResponse,
    },
    state::AppState,
};

pub async fn create_execution(
    State(state): State<AppState>,
    Json(request): Json<CreateExecutionRequest>,
) -> Result<Json<CreateExecutionResponse>, StatusCode> {

    let execution = state
        .execution_service
        .create_execution(request.version_id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(CreateExecutionResponse {
        execution_id: execution.id,
    }))
}

pub async fn get_execution(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExecutionResponse>, StatusCode> {

    let execution = state
        .execution_service
        .get_execution(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let execution = execution.read().await;

    Ok(Json(ExecutionResponse {
        id: execution.id,
        version_id: execution.agent_version_id,
        task_id: execution.task_id,
        status: format!("{:?}", execution.status),
        result: execution.result.clone(),
    }))
}

pub async fn list_executions(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExecutionResponse>>, StatusCode> {

    let executions = state
        .execution_service
        .list_executions()
        .await;

    let mut response = Vec::with_capacity(executions.len());

    for execution in executions {

        let execution = execution.read().await;

        response.push(ExecutionResponse {
            id: execution.id,
            version_id: execution.agent_version_id,
            task_id: execution.task_id,
            status: format!("{:?}", execution.status),
            result: execution.result.clone(),
        });
    }

    Ok(Json(response))
}