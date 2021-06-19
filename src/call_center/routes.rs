use std::collections::HashMap;

use super::models::*;
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Method for displaying agent details
#[get("/fetch-agent-by-id")]
async fn agent_by_id(
    pool: web::Data<DbPool>,
    data: web::Query<HashMap<String, i32>>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool.");
    let agent_id = *data
        .get(&String::from("agent-id"))
        .expect("Invalid keyword! Please pass agent-id to get details.");
    let agent_name = Agent::get_agent_details(agent_id, &connection)
        .expect("Error while getting `agent_details`.");

    Ok(HttpResponse::Ok().json(agent_name))
}

// Method for displaying call details
#[get("/fetch-call-by-id")]
async fn call_by_id(
    pool: web::Data<DbPool>,
    data: web::Query<HashMap<String, i32>>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool.");
    let call_id = *data
        .get(&String::from("call-id"))
        .expect("Invalid keyword! Please pass call-id to get details.");
    let call_details = CallRecords::get_call_details(call_id, &connection)
        .expect("Error while getting `call_details`.");

    Ok(HttpResponse::Ok().json(call_details))
}

// Method for displaying call details by agent
#[post("/fetch-calls-by-agent")]
async fn calls_by_agent(
    pool: web::Data<DbPool>,
    data: web::Json<AgentQuery>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool.");
    let agent_call_details = AgentCallRecords::get_agent_call_details(data, &connection)
        .expect("Error while getting `agent_call_details`.");

    Ok(HttpResponse::Ok().json(agent_call_details))
}

// Method for displaying agent daily call volume
#[post("/daily-call-volume")]
async fn daily_call_volume(
    pool: web::Data<DbPool>,
    data: web::Json<AgentCallQuery>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool.");
    let agent_call_volume = AgentCallVolume::get_agent_call_details(data, &connection)
        .expect("Error while getting `agent_call_details`.");

    Ok(HttpResponse::Ok().json(agent_call_volume))
}
