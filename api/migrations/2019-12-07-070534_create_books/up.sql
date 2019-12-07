-- Your SQL goes here
create TABLE books(
id SERIAL PRIMARY KEY,
title VARCHAR NOT NULL,
author VARCHAR NOT NULL,
published BOOLEAN NOT NULL DEFAULT 'f'
)