CREATE TABLE "node" (
    id SERIAL PRIMARY KEY,
    node_label TEXT NOT NULL,
    graph_id INT REFERENCES graph(id) ON DELETE CASCADE NOT NULL,
    x_coord DOUBLE PRECISION NOT NULL,
    y_coord DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);