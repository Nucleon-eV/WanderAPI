CREATE TABLE IF NOT EXISTS pois
(
    id           SERIAL PRIMARY KEY,
    hiking_trail SERIAL,
    name         VARCHAR NOT NULL,
    description  TEXT NOT NULL,
    location     VARCHAR NOT NULL
);