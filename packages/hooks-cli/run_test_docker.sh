#!/bin/sh

docker run -it \
  --init \
  -p 1993:1993 \
  -v $PWD:/app \
  hooks-cli:latest \
  test \
  --allow-all \
  /app/tests/*_test.ts
