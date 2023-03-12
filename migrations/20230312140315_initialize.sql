-- Add migration script here
CREATE table pastes (
	id Uuid PRIMARY KEY,
	content text NOT NULL,
	created_at timestamp not null default CURRENT_TIMESTAMP
);