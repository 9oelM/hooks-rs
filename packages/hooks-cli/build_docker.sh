#!/bin/sh

docker buildx create --bootstrap --use --name builder

docker build --platform linux/arm64/v8 -t hooks-cli-test:latest -f Dockerfile.arm64 .
# docker build --platform linux/amd64 -t hooks-cli-test:latest .
