use super::*;
use crate::schema::*;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable, Serialize, Queryable, Debug, Clone)]
#[table_name = "call_records"]
pub struct CallRecord {
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
pub struct CallRecords {
    pub calls: Vec<CallRecord>,
}

impl CallRecords {
    pub fn get_call_details(id: i32, connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::call_records::dsl::*;
        let calls: Vec<CallRecord> = call_records
            .filter(call_id.eq(id))
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
            .load(connection)
            .expect("Error while loading data from `call_records` table.");

        Ok(Self { calls })
    }
}
