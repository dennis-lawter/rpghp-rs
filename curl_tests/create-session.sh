#!/usr/bin/env bash

URL=http://0.0.0.0:8080/api/v1/session/
METHOD=POST

echo $METHOD $URL

curl --request $METHOD \
  -sSi \
  --url $URL \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: curl'
echo ""
