# Shoping API

# Initialize POSTGRES Database

```sh
Check .env file for Host IP and Port Number

run below command to setup diesel
diesel setup

```
## Available routes
# GET /fetch-agent-by-id"
# GET /fetch-call-by-id
# POST /fetch-calls-by-agent
# POST /daily-call-volume

# For fetching agent, use below command

This API is used to fetch agent name by registered agent id

```sh
curl --location --request GET 'localhost:8080/fetch-agent-by-id?agent-id=1'

```

# For fetching call details, use below command

This API is used to fetch call details by registered call id

```sh
curl --location --request GET 'localhost:8080/fetch-call-by-id?call-id=34648'

```

# For fetching call details by agent, use below command

This API is used to fetch calls by agent having skip and limit

```sh
curl --location --request POST 'localhost:8080/fetch-calls-by-agent' \
--header 'Content-Type: application/json' \
--data-raw '{
    "agent-id": 1,
    "skip": 2,
    "limit": 30
}'

```

# For fetching call details by agent, use below command

This API is used to fetch calls by agent having skip and limit

```sh
curl --location --request POST 'localhost:8080/fetch-calls-by-agent' \
--header 'Content-Type: application/json' \
--data-raw '{
    "agent-id": 1,
    "skip": 2,
    "limit": 30
}'

```

# For fetching daily call volume of agents between particular dates, use below command

This API is used to fetch call volume of agent between two dates

```sh
curl --location --request POST 'localhost:8080/daily-call-volume' \
--header 'Content-Type: application/json' \
--data-raw '{
    "start-date": 990101,
    "end-date": 990103,
    "agent-id": 1
}'

```

# To run Services

This run.sh script contains the host_url and connection string of Postgres Database

```sh
./run.sh 

```
