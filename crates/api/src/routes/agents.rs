
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use domain::Agent;

use crate::{
    models::{
        CreateAgentRequest,
        CreateAgentResponse,
    },
    state::AppState,
};

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