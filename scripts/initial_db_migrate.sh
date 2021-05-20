#!/usr/bin/env bash

export DATABASE_URL=postgres://postgres:yourpassword@localhost:5432/newsletters
sqlx migrate add create_subscriptions_table