#!/usr/bin/env bash
curl --request POST \
  -sSi \
  --url http://0.0.0.0:8080/session/ \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: curl'
echo ""
