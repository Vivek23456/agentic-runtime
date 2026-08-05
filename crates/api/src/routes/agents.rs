use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;
use domain::Agent;

use crate::{
    models::{   
        CreateAgentRequest,
        CreateAgentResponse,
        AgentResponse,
    },
    state::AppState,
};

pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AgentResponse>, StatusCode> {

    let agent = state
        .agents
        .get(id)
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

    let agent = Agent::new(
        request.name,
        request.description,
        request.framework,
        request.language,
        request.repository,
    );

    let id = agent.id;

    state.agents.insert(agent).await;

    Ok(Json(CreateAgentResponse {
        agent_id: id,
    }))
}

pub async fn list_agents(
    State(state): State<AppState>,
) -> Result<Json<Vec<AgentResponse>>, StatusCode> {

    let agents = state.agents.list().await;

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