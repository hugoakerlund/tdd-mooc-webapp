#!/usr/bin/env bash
set -euxo pipefail

curl --silent --show-error --fail http://localhost:3000 > /dev/null
front_end_response_status=$?
test "$front_end_response_status" = 0

backend_response=$(curl --silent --show-error http://localhost:3001)
test "$backend_response" = "{\"text\":\"Welcome to Rust API\"}"

: OK