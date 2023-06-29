# FactoryCI

Rust Applications
MySQL Database

A workflow can be made up of multiple pipelines
A pipeline can be made up of multiple stages
A stage can perform multiple executions

## API

Webhook Handler

1. Handle webhooks from GitHub
2. POST /hooks/github
   1. Parse webhook payload
   2. Read project information from database
   3. Call the orchestrator to queue a new workflow

Project Manager

1. GET /project?id=123
2. POST /project
3. PUT /project?id=123
4. DELETE /project?id=123

| Field | Type | Description |
| ----- | ---- | ----------- |
| id | uuid | Project ID |
| url | string | GitHub URL |
| webhook_secret | string | GitHub Webhook Secret used to verify x-hub-signature-256 header |

Orchestrator

1. Queue new workflows (function called by webhook handler)
   1. Use the project information to provision new build agent
   2. Use Consul for service discovery
   3. Use Kubernetes for container (build agent) orchestration

## Build Agent

The build agent is an image that contains the following capabilities:

1. Checkout the source code from GitHub
2. Parse the .factory directory
   1. All *.hcl files are parsed and converted into a single configuration
3. Determine which pipelines are to be executed (checking out the filter stuff)
4. Execute pipelines according to stages
5. Execute the stages
