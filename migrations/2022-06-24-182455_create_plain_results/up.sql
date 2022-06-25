CREATE TABLE plain_results (
    plain_result_id UUID PRIMARY KEY,
    plain_result_content BYTEA NOT NULL,
    plain_result_created_at TIMESTAMP NOT NULL,
    task_id UUID REFERENCES tasks (task_id) NOT NULL
);