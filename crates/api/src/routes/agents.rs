use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{
    models::{
        AgentResponse,
        CreateAgentRequest,
        CreateAgentResponse,
    },
    state::AppState,
};

pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AgentResponse>, StatusCode> {
    let agent = state
        .agent_service
        .get_agent(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let agent = agent.read().await;

    Ok(Json(AgentResponse {
        id: agent.id,
        name: agent.name.clone(),
        description: agent.description.clone(),
        framework: agent.framework,
        language: agent.language,
        repository: agent.repository.clone(),
    }))
}

pub async fn create_agent(
    State(state): State<AppState>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<Json<CreateAgentResponse>, StatusCode> {
    let agent = state
        .agent_service
        .create_agent(
            request.name,
            request.description,
            request.framework,
            request.language,
            request.repository,
        )
        .await;

    Ok(Json(CreateAgentResponse {
        agent_id: agent.id,
    }))
}

pub async fn list_agents(
    State(state): State<AppState>,
) -> Result<Json<Vec<AgentResponse>>, StatusCode> {
    let agents = state
        .agent_service
        .list_agents()
        .await;

    let mut response = Vec::with_capacity(agents.len());

    for agent in agents {
        let agent = agent.read().await;

        response.push(AgentResponse {
            id: agent.id,
            name: agent.name.clone(),
            description: agent.description.clone(),
            framework: agent.framework,
            language: agent.language,
            repository: agent.repository.clone(),
        });
    }

    Ok(Json(response))
}