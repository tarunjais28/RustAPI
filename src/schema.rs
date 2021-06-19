table! {
    agents (agent_id) {
        agent_id -> Int4,
        agent_name -> Text,
    }
}

table! {
    call_records (call_id, customer_id, date, ivr_in, ivr_out, agent_id) {
        call_id -> Int4,
        customer_id -> Int8,
        priority -> Int4,
        #[sql_name = "type"]
        typ -> Text,
        date -> Date,
        ivr_in -> Time,
        ivr_out -> Time,
        ivr_time -> Int4,
        q_start -> Time,
        q_exit -> Time,
        q_time -> Int4,
        outcome -> Text,
        agent_start -> Time,
        agent_exit -> Time,
        ser_time -> Int4,
        agent_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(agents, call_records,);
