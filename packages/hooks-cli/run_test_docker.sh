#!/bin/sh

docker run -it \
  --init \
  -p 1993:1993 \
  -v $PWD:/app \
  hooks-cli-test:latest \
  test \
  --allow-net \
  --allow-write \
  --allow-run \
  --allow-read \
  --allow-env \
  --allow-sys \
  /app/tests/*_test.ts
