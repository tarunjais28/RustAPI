CREATE TABLE call_records (
  call_id int not null,
  customer_id bigint not null,
  priority int not null,
  type text not null,
  date date not null,
  ivr_in time not null,
  ivr_out time not null,
  ivr_time int not null,
  q_start time not null,
  q_exit time not null,
  q_time int not null,
  outcome text not null,
  agent_start time not null,
  agent_exit time not null,
  ser_time int not null,
  agent_id int not null,
  PRIMARY KEY (call_id, customer_id,date,ivr_in,ivr_out,agent_id)
);

CREATE TABLE agents (
  agent_id int not null,
  agent_name text not null,
  PRIMARY KEY (agent_id)
);
