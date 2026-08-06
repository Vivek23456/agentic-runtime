use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    models::{
        CreateVersionRequest,
        CreateVersionResponse,
        VersionResponse,
    },
    state::AppState,
};

pub async fn create_version(
    State(state): State<AppState>,
    Json(request): Json<CreateVersionRequest>,
) -> Result<Json<CreateVersionResponse>, StatusCode> {
    let version = state
        .version_service
        .create_version(
            request.agent_id,
            request.version,
            request.description,
            request.prompt,
            request.model,
            request.temperature,
        )
        .await;

    Ok(Json(CreateVersionResponse {
        version_id: version.id,
    }))
}

pub async fn get_version(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<VersionResponse>, StatusCode> {
    let version = state
        .version_service
        .get_version(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let version = version.read().await;

    Ok(Json(VersionResponse {
        id: version.id,
        agent_id: version.agent_id,
        version: version.version.clone(),
        description: version.description.clone(),
        prompt: version.prompt.clone(),
        model: version.model.clone(),
        temperature: version.temperature,
    }))
}

pub async fn list_versions(
    State(state): State<AppState>,
) -> Result<Json<Vec<VersionResponse>>, StatusCode> {
    let versions = state.version_service.list_versions().await;

    let mut response = Vec::with_capacity(versions.len());

    for version in versions {
        let version = version.read().await;

        response.push(VersionResponse {
            id: version.id,
            agent_id: version.agent_id,
            version: version.version.clone(),
            description: version.description.clone(),
            prompt: version.prompt.clone(),
            model: version.model.clone(),
            temperature: version.temperature,
        });
    }

    Ok(Json(response))
}