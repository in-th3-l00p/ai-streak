#!/usr/bin/env bash

HOST="${API_HOST:-http://localhost}"
PORT="${API_PORT:-8080}"

URL="$HOST:$PORT/api/login"

echo "POST $URL"

curl -s -X POST "$URL" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "intheloop",
    "password": "password"
  }'
echo