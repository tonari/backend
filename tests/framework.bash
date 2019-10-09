# Common functions used for our tests

# use this instead of echo inside tests
log() {
  local BLUE='\033[0;34m'
  local RESET='\033[0m'
  echo -e "$BLUE"[test] "$*""$RESET"
}

container-running() {
  [ "$(docker inspect -f '{{.State.Running}}' "$1")" = "true" ]
}

container-stop() {
  # ignore if there is an error (which means that the container is already stopped)
  docker stop "$1" 2>&1>/dev/null || true
}

containers-stop() {
  [ -n "${TONARI:-}" ] && container-stop "$TONARI"
  [ -n "${MONGO:-}" ] && container-stop "$MONGO"
}

containers-run() {
  export TONARI_SOURCE_ID=${TONARI_SOURCE_ID:-00000000000000000000000000000000}
  export TONARI_IMAGE_URL_PREFIX=${TONARI_IMAGE_URL_PREFIX:-https://tonari.app/api/images/}
  export TONARI_IMAGE_PATH=${TONARI_IMAGE_PATH:-/images}
  export TONARI_DATABASE_NAME=${TONARI_DATABASE_NAME:-tonari}
  export ROCKET_DATABASES=${ROCKET_DATABASES:-'{sanitary_facilities={url="mongodb://localhost:27017/sanitary_facilities"}}'}
  export ROCKET_PORT=${ROCKET_PORT:-8000}
  ROCKET_SECRET_KEY_DEFAULT=$(openssl rand -base64 32)
  export ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY:-$ROCKET_SECRET_KEY_DEFAULT}
  export MONGO=$(docker run -d -p 27017:27017 mongo --wiredTigerCacheSizeGB 1.5)
  export TONARI=$(docker run -d -eTONARI_{SOURCE_ID,IMAGE_URL_PREFIX,IMAGE_PATH} -eROCKET_{DATABASES,PORT} --network=host tonari/backend)
}

# bats setup
setup() {
  containers-run
}

# bats teardown
teardown() {
  containers-stop
}
