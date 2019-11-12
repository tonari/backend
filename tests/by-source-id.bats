#!/usr/bin/env bats

load framework

# This test ensures that
#   1. all facilities with the same source get returned when searching for that source
#   2. only facilities with that source get returned
@test "Retrieve by source Id" {
  # add facilities with the same source
  local request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11,
    "name": "Foobar"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  local request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 73,
    "lon": 22,
    "name": "Foobar 2"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  local request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": -3,
    "lon": 17,
    "name": "Foobar 3"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  # add a new facility with a difference source id
  local request=$(cat <<JSON
{
    "createNewFacility": false,
    "id": {
        "sourceId": "notTonariSourceId",
        "originalId": "123456"
    },
    "lat": 48,
    "lon": -31,
    "name": "Not foobar"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  local result=$(request get "facilities/by-source-id/$TONARI_SOURCE_ID")

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "3"
  field-equals "$result" '.features | length' "3"
  field-equals "$result" .features[0].properties.sourceId "$TONARI_SOURCE_ID"
  field-equals "$result" .features[1].properties.sourceId "$TONARI_SOURCE_ID"
  field-equals "$result" .features[2].properties.sourceId "$TONARI_SOURCE_ID"

  local result=$(request get facilities/by-source-id/notTonariSourceId)

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"
  field-equals "$result" '.features | length' "1"
  field-equals "$result" .features[0].properties.sourceId "notTonariSourceId"
  field-equals "$result" .features[0].properties.name "Not foobar"
}
