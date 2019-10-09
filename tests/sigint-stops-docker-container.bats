#!/usr/bin/env bats

load framework

# Test if a SIGINT signal stops the Docker container
@test "SIGINT stops Docker container" {
  # wait so that the container is stopped if there are any startup problems
  sleep 1

  if ! container-running $TONARI; then
    log Tonari failed to start:
    docker logs $TONARI
    echo Tonari is running > /dev/null
    return 1
  fi

  docker kill --signal=SIGINT $TONARI > /dev/null
  sleep 4
  if container-running $TONARI; then
    log Tonari failed to be killed
    false
  fi
}
