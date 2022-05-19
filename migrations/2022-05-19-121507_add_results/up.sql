CREATE TABLE results (
    result_id UUID PRIMARY KEY,
    result_content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    task_id UUID REFERENCES tasks (task_id) NOT NULL
);