# For local and CI testing
#!/bin/sh

docker run \
  --init \
  -p 1993:1993 \
  -v $PWD:/app \
  hooks-cli:latest \
  test \
  --allow-all
