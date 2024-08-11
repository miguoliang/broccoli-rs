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

CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON vertex
    FOR EACH ROW
    EXECUTE PROCEDURE set_timestamp();

CREATE TABLE edge (
    id SERIAL PRIMARY KEY,
    from_vertex_id INT NOT NULL,
    from_vertex_type VARCHAR(255) NOT NULL,
    to_vertex_id INT NOT NULL,
    to_vertex_type VARCHAR(255) NOT NULL,
    label VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255) NOT NULL
);

CREATE INDEX edge_from_vertex ON edge (from_vertex_id);
CREATE INDEX edge_to_vertex ON edge (to_vertex_id);
CREATE INDEX edge_label ON edge (label);
CREATE INDEX edge_from_vertex_type ON edge (from_vertex_type);
CREATE INDEX edge_to_vertex_type ON edge (to_vertex_type);
CREATE UNIQUE INDEX edge_from_to_label ON edge (from_vertex_id, to_vertex_id, label);
CREATE INDEX edge_created_at ON edge (created_at);
CREATE INDEX edge_updated_at ON edge (updated_at);
CREATE INDEX edge_created_by ON edge (created_by);
CREATE INDEX edge_updated_by ON edge (updated_by);

CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON edge
    FOR EACH ROW
    EXECUTE PROCEDURE set_timestamp();