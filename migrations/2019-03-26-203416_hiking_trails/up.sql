CREATE TABLE IF NOT EXISTS hiking_trails
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    location VARCHAR NOT NULL
);