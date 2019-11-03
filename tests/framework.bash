# Common functions used for our tests

container-stopped() {
  [ "$(docker inspect -f '{{.State.Running}}' "$1")" = "false" ]
}

container-stop() {
  # ignore if there is an error (which means that the container is already stopped)
  docker stop "$1" >/dev/null 2>&1 || true
}

container-ip() {
  docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$1"
}

containers-stop() {
  [ -n "${TONARI:-}" ] && container-stop "$TONARI"
  [ -n "${MONGO:-}" ] && container-stop "$MONGO"
}

containers-run() {
  export MONGO
  MONGO=$(docker run --rm -d mongo --wiredTigerCacheSizeGB 1.5)
  MONGO_IP=$(container-ip "$MONGO")

  export TONARI_SOURCE_ID=${TONARI_SOURCE_ID:-00000000000000000000000000000000}
  export TONARI_IMAGE_URL_PREFIX=${TONARI_IMAGE_URL_PREFIX:-https://tonari.app/api/images/}
  export TONARI_IMAGE_PATH=${TONARI_IMAGE_PATH:-/images}
  export TONARI_DATABASE_NAME=${TONARI_DATABASE_NAME:-tonari}
  export TONARI_INITIALIZE_DB=${TONARI_INITIALIZE_DB:-1}
  export ROCKET_DATABASES=${ROCKET_DATABASES:-"{sanitary_facilities={url=\"mongodb://$MONGO_IP:27017/sanitary_facilities\"}}"}
  export ROCKET_PORT=${ROCKET_PORT:-8000}
  ROCKET_SECRET_KEY_DEFAULT=$(openssl rand -base64 32)
  export ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY:-$ROCKET_SECRET_KEY_DEFAULT}
  export TONARI
  TONARI=$(docker run --rm -d -eTONARI_{SOURCE_ID,IMAGE_URL_PREFIX,IMAGE_PATH,INITIALIZE_DB} -eROCKET_{DATABASES,PORT,SECRET_KEY} tonari/backend)
  export TONARI_IP
  TONARI_IP=$(container-ip "$TONARI")
}

request() {
  local method=$1
  local path_=$2
  local request=${3:-}

  if [ "$method" = post ]; then
    local args=('-d' "$request" '-H' 'Content-Type: application/json')
  elif [ "$method" = "post-multipart" ]; then
    local args=('-F' "$request")
  fi

  curl -sS --max-time 5 --connect-timeout 5 "${args[@]}" "http://$TONARI_IP:8000/$path_"
}

expect() {
  local method=$1
  local path_=$2
  local expected_result=$3
  local request=${4:-}

  local result
  if ! result=$(request "$method" "$path_" "$request"); then
    return 1
  fi

  diff <(echo "$result" | jq -S) <(echo "$expected_result" | jq -S)
}

field-exists() {
  local jsonObject=$1
  local jqQuery=$2
  [[ $(echo "$jsonObject" | jq "$jqQuery") != null ]]
}

field-equals() {
  local jsonObject=$1
  local jqQuery=$2
  local expectedValue=$3
  diff <(echo "$jsonObject" | jq -r "$jqQuery") <(echo "$expectedValue") || (echo "JSON field differs: \`$jqQuery\`. See the diff above." && false)
}

extract-field() {
  local jsonObject=$1
  local jqQuery=$2
  local datatype=${3:-}

  if [ "$datatype" = "string" ]; then
    # remove the quotation marks
    echo "$jsonObject" | jq "$jqQuery" | head -c -2 | tail -c +2
  else
    echo "$jsonObject" | jq "$jqQuery"
  fi
}

is-json() {
  local content=$1
  if ! echo "$content" | jq >/dev/null; then
    return 1
  else
    # an empty response shouldn't be classified as valid JSON
    [ "$content" ]
  fi
}

# milliseconds since epoch
now() {
  date +%s%3N
}

# repeat a command until it succeeds, or until timeout
await() {
  local timeout=$1; shift
  local command=("$@")

  local startTime
  startTime=$(now)
  while (( $(now) - startTime < timeout )); do
    if "${command[@]}" >/dev/null; then
      return 0
    fi
    sleep 0.1
  done
  return 1
}

# Wait until the HTTP server of the Docker container is reachable. We give up
# after a certain amount of waiting time and assume that the HTTP server failed
# to start.
await-http() {
  # the requested path has to be a path the server serves so that the successful startup test doesn't fail
  await 5000 curl -sS --max-time 1 --connect-timeout 1 "http://$TONARI_IP:8000/facilities/by-radius/0/0/0"
}

# bats setup
setup() {
  containers-run
  await-http
}

# bats teardown
teardown() {
  containers-stop
}
