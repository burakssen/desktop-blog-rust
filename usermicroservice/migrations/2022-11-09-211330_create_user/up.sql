-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR (25) NOT NULL UNIQUE,
    password VARCHAR (25) NOT NULL,
    mail VARCHAR (25) NOT NULL UNIQUE
);