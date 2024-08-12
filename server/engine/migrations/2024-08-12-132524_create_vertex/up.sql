-- Trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE vertex (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX vertex_name_type ON vertex (name, type);
CREATE INDEX vertex_type ON vertex (type);
CREATE INDEX vertex_name ON vertex (name);
CREATE INDEX vertex_created_at ON vertex (created_at);
CREATE INDEX vertex_updated_at ON vertex (updated_at);
CREATE INDEX vertex_created_by ON vertex (created_by);
CREATE INDEX vertex_updated_by ON vertex (updated_by);

CREATE TRIGGER update_vertex_updated_at
BEFORE UPDATE ON vertex
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();