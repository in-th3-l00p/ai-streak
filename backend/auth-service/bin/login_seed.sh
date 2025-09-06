#!/usr/bin/env bash

host="${API_HOST:-http://localhost}"
port="${API_PORT:-8080}"

url="$host:$port/api/login"

jwt=$(curl -s -X POST "$url" \
  -H "Content-Type: application/json" \
  -d '{ "username": "intheloop", "password": "password" }' \
  | jq -r ".jwt")

if [[ -z "$jwt" || "$jwt" == "null" ]]; then
  echo "no jwt returned"
  exit 1
fi

export JWT="$jwt"
echo "jwt stored"