CREATE TABLE implants (
    implant_id UUID PRIMARY KEY,
    public_key TEXT NOT NULL,
    server_private_key TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    external_ip_address TEXT,
    internal_ip_address TEXT,
    os_type TEXT,
    machine_user TEXT,
    machine_name TEXT,
    process_name TEXT,
    pid INTEGER,
    architecture INTEGER
);