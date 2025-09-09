#!/usr/bin/env bash
set -euo pipefail

HOST="${API_HOST:-http://localhost}"
PORT="${API_PORT:-8080}"

URL="$HOST:$PORT/api/auth/current"

if [[ -z "${JWT:-}" ]]; then
  echo "jwt not found"
  exit 1
fi

RESP=$(
  curl -s -X GET "$URL" \
    -H "Authorization: Bearer $JWT" \
    -H "Content-Type: application/json"
)

echo "$RESP"