-- Add migration script here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	created_at timestamp NOT NULL default CURRENT_TIMESTAMP,
	snowflake varchar(25) NOT NULL
);