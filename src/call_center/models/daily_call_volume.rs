use super::*;
use crate::schema::*;
use actix_web::web;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Struct to read Json File from input
#[derive(Serialize, Deserialize)]
pub struct AgentCallQuery {
    #[serde(rename = "start-date")]
    pub start_date: i32,
    #[serde(rename = "end-date")]
    pub end_date: i32,
    #[serde(rename = "agent-id")]
    pub agent_id: i32,
}

// Struct to extract data from table
#[derive(Deserialize, Insertable, Serialize, Queryable, Debug, Clone)]
#[table_name = "call_records"]
pub struct AgentCallData {
    pub date: NaiveDate,
    pub ivr_time: i32,
    pub q_time: i32,
    pub ser_time: i32,
}

// Struct containg call volume of an Agent
#[derive(Serialize, Deserialize)]
pub struct CallVolume {
    pub date: i32,
    pub call_count: i32,
    pub tot_ivr_time: i32,
    pub tot_q_time: i32,
    pub tot_ser_time: i32,
}

impl CallVolume {
    fn new(date: NaiveDate, aggr_val: AggregatedValues) -> Self {
        Self {
            date: get_i32_from_date(date),
            call_count: aggr_val.call_count,
            tot_ivr_time: aggr_val.ivr_time,
            tot_q_time: aggr_val.q_time,
            tot_ser_time: aggr_val.ser_time,
        }
    }
}

// Struct to contain aggregated call volumes of agents
#[derive(Serialize, Deserialize)]
pub struct AgentCallVolume {
    pub data: Vec<CallVolume>,
}

// Struct used for Aggregating Data
struct AggregatedValues {
    pub call_count: i32,
    pub ivr_time: i32,
    pub q_time: i32,
    pub ser_time: i32,
}

impl AggregatedValues {
    fn new(call_data: &AgentCallData) -> Self {
        Self {
            call_count: 1,
            ivr_time: call_data.ivr_time,
            q_time: call_data.q_time,
            ser_time: call_data.ser_time,
        }
    }

    fn add(&mut self, aggr_val: &AggregatedValues) {
        self.call_count += 1;
        self.ivr_time += aggr_val.ivr_time;
        self.q_time += aggr_val.q_time;
        self.ser_time += aggr_val.ser_time;
    }
}

impl AgentCallVolume {
    pub fn get_agent_call_details(
        agent_query: web::Json<AgentCallQuery>,
        connection: &PgConnection,
    ) -> Result<Self, Error> {
        use crate::schema::call_records::dsl::*;

        let start_date = get_date_from_i32(agent_query.start_date);
        let end_date = get_date_from_i32(agent_query.end_date);

        // Extracting call data from call_record table
        let agent_call_data: Vec<AgentCallData> = call_records
            .filter(agent_id.eq(agent_query.agent_id))
            .filter(date.between(start_date, end_date))
            .select((date, ivr_time, q_time, ser_time))
            .load(connection)
            .expect("Error while loading data from `call_records` table.");

        Ok(AgentCallVolume {
            data: aggregate(agent_call_data),
        })
    }
}

// Method to calculate agggregate call volumes for an agent
fn aggregate(agent_call_data: Vec<AgentCallData>) -> Vec<CallVolume> {
    let mut aggr_data: HashMap<NaiveDate, AggregatedValues> = HashMap::new();

    for data in agent_call_data {
        let aggr_val = AggregatedValues::new(&data);
        aggr_data
            .entry(data.date)
            .and_modify(|data| data.add(&aggr_val))
            .or_insert(aggr_val);
    }

    let mut agent_aggr_call_data: Vec<CallVolume> = Vec::new();
    for (date, aggr_data) in aggr_data.drain() {
        agent_aggr_call_data.push(CallVolume::new(date, aggr_data));
    }
    agent_aggr_call_data
}

// Method to convert integer into date
fn get_date_from_i32(date: i32) -> NaiveDate {
    match NaiveDate::parse_from_str(&date.to_string(), "%y%m%d") {
        Ok(parsed_date) => parsed_date,
        Err(error) => panic!(
            "Invalid date passed:`{}`, expected format: yymmdd. `{}`.",
            date, error
        ),
    }
}

// Method to convert date into integer
fn get_i32_from_date(date: NaiveDate) -> i32 {
    date.format("%y%m%d")
        .to_string()
        .parse::<i32>()
        .expect("Error while converting date into integer.")
}
