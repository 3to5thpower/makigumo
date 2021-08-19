-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR(100) PRIMARY KEY NOT NULL, 
    name VARCHAR(100) NOT NULL
);

INSERT INTO users (id, name) VALUES ('admin', 'admin'), ('test1@example.jp', 'user1'), ('test2@example.jp','user2');

