-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(64) NOT NULL
);

INSERT INTO users (name) VALUES ('admin'), ('user1'), ('user2');

