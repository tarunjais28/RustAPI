use super::{Deserialize, Serialize};
use crate::schema::*;
use diesel::{pg::PgConnection, prelude::*, result::Error};

#[derive(Deserialize, Insertable, Serialize, Queryable, Debug, Clone)]
#[table_name = "agents"]
pub struct Agent {
    #[serde(rename = "name")]
    pub agent_name: String,
}

impl Agent {
    pub fn get_agent_details(agent_id: i32, connection: &PgConnection) -> Result<Self, Error> {
        let name = crate::schema::agents::dsl::agents
            .filter(crate::schema::agents::agent_id.eq(agent_id))
            .select(crate::schema::agents::dsl::agent_name)
            .first(connection)
            .expect("Error while loading data from `agents` table.");

        Ok(Self { agent_name: name })
    }
}
