#!/usr/bin/env bash

export DATABASE_URL=postgres://postgres:yourpassword@localhost:5432/newsletters
sqlx database create
sqlx migrate run