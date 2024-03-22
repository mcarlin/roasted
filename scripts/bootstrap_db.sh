#!/usr/bin/env sh

HOST=$1
PORT=$2

db_exists=$(psql "postgres://postgres:postgres@localhost:5433" -tc "SELECT 1 FROM pg_database WHERE datname = 'roasted'" | grep -c 1)
if [ "$db_exists" -gt "0" ]; then
  printf "You're about to wipe out your existing 'roasted' database, are you absolutely sure (y/n)? "
  read -r answer

  if [ "$answer" != "${answer#[Nn]}" ] ;then
    exit 0
  fi
fi

psql "postgres://postgres:postgres@${HOST}:${PORT}" -c "drop database if exists roasted;"
psql "postgres://postgres:postgres@${HOST}:${PORT}" -c "create database roasted;"

psql "postgres://postgres:postgres@${HOST}:${PORT}/roasted" \
  -f bootstrap_db.sql \
  -vadmin_pass="${DBADMIN_PASS}" \
  -vapp_pass="${APP_PASS}"
