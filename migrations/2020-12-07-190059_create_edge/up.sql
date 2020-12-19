CREATE TABLE "edge" (
    id SERIAL PRIMARY KEY,
    edge_label TEXT NOT NULL,
    source_id INT REFERENCES node(id) ON DELETE CASCADE NOT NULL,
    target_id INT REFERENCES node(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);