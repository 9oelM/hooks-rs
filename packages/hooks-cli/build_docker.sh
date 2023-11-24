#!/bin/bash

# usage: ./build_docker.sh <platform>

PLATFORM="$1"
DOCKERFILE_POSTFIX=""

if [ "$PLATFORM" == "linux/arm64/v8" ]; then
    DOCKERFILE_POSTFIX="arm64"
elif [ "$PLATFORM" == "linux/amd64" ]; then
    DOCKERFILE_POSTFIX="amd64"
else
    echo "Unsupported platform: $PLATFORM"
    exit 1
fi

docker build --platform "${PLATFORM}" -t hooks-cli:latest -f "Dockerfile.${DOCKERFILE_POSTFIX}" .
