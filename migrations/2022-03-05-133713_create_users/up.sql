-- Your SQL goes here-- 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name varchar(255),
    last_name varchar(255),
    email varchar(255),
    password varchar(255),
    file_url varchar(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);