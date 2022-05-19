CREATE TABLE tasks (
    task_id UUID PRIMARY KEY,
    task TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    status TEXT NOT NULL,
    implant_id UUID REFERENCES implants (implant_id) NOT NULL
);