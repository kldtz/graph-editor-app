CREATE TABLE "graph" (
    id SERIAL PRIMARY KEY,
    graph_label TEXT UNIQUE NOT NULL,
    translate_x DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    translate_y DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    scale DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);