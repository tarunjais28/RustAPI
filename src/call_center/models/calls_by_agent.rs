use super::*;
use crate::schema::*;
use actix_web::web;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

// Struct used to read input Json Data
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentQuery {
    #[serde(rename = "agent-id")]
    pub agent_id: i32,
    pub skip: i64,
    pub limit: i64,
}

#[derive(Deserialize, Insertable, Serialize, Queryable, Debug, Clone)]
#[table_name = "call_records"]
pub struct AgentCallRecord {
    pub call_id: i32,
    pub customer_id: i64,
    pub priority: i32,
    #[serde(rename = "type")]
    pub typ: String,
    pub date: NaiveDate,
    pub ivr_in: NaiveTime,
    pub ivr_out: NaiveTime,
    pub ivr_time: i32,
    pub q_start: NaiveTime,
    pub q_exit: NaiveTime,
    pub q_time: i32,
    pub outcome: String,
    pub agent_start: NaiveTime,
    pub agent_exit: NaiveTime,
    pub ser_time: i32,
    pub agent_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentCallRecords {
    pub calls: Vec<AgentCallRecord>,
}

impl AgentCallRecords {
    pub fn get_agent_call_details(
        agent_query: web::Json<AgentQuery>,
        connection: &PgConnection,
    ) -> Result<Self, Error> {
        use crate::schema::call_records::dsl::*;

        let calls: Vec<AgentCallRecord> = call_records
            .filter(agent_id.eq(agent_query.agent_id))
            .select((
                call_id,
                customer_id,
                priority,
                typ,
                date,
                ivr_in,
                ivr_out,
                ivr_time,
                q_start,
                q_exit,
                q_time,
                outcome,
                agent_start,
                agent_exit,
                ser_time,
                agent_id,
            ))
            .offset(agent_query.skip)
            .limit(agent_query.limit)
            .load(connection)
            .expect("Error while loading data from `call_records` table.");

        Ok(Self { calls })
    }
}
