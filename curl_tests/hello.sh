#!/usr/bin/env bash
curl --request GET \
  -sSi \
  --url http://0.0.0.0:8080/hello/test \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: curl'
echo ""
