CREATE TABLE implants (
    implant_id UUID PRIMARY KEY,
    public_key TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);