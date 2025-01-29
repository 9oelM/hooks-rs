# For production; push to Docker Hub
#!/bin/bash

docker buildx build --platform linux/amd64,linux/arm64 -t "9oel/hooks-cli:latest" -f "Dockerfile.release" --push .
