CREATE TABLE tasks (
    task_id UUID PRIMARY KEY,
    task TEXT NOT NULL,
    task_created_at TIMESTAMP NOT NULL,
    task_status TEXT NOT NULL,
    result_content TEXT NULL,
    result_nonce TEXT NULL,
    result_created_at TIMESTAMP NULL,
    implant_id UUID REFERENCES implants (implant_id) NOT NULL
);