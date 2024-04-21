CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    td_desc VARCHAR NOT NULL,
    td_status boolean NOT NULL DEFAULT FALSE
)