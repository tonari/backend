#!/usr/bin/env bats

load framework

# Test if a SIGINT signal stops the Docker container
@test "SIGINT stops Docker container" {
  docker kill --signal=SIGINT "$TONARI" > /dev/null
  await 4000 container-stopped "$TONARI"
}
