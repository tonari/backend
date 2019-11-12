#!/usr/bin/env bats

load framework

# This test ensures that
#   1. facilities get assigned an id
#   2. facilities are reachable by their id
@test "Retrieve by Id" {
  # add facility
  request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11,
    "name": "Foobar"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  # get the orignalId
  result=$(request get facilities/by-tile/0/0/0)

  local originalId=$(extract-field "$result" .features[0].properties.originalId)

  result=$(request get "facilities/by-id/$TONARI_SOURCE_ID/$originalId")

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"
  field-equals "$result" '.features | length' "1"
  field-equals "$result" .features[0].type "Feature"
  field-exists "$result" .features[0].properties.originalId
  field-equals "$result" .features[0].properties.sourceId "$TONARI_SOURCE_ID"
  field-equals "$result" .features[0].properties.category "toilets"
  field-equals "$result" .features[0].properties.name "Foobar"
  field-equals "$result" .features[0].geometry.coordinates[0] "11"
  field-equals "$result" .features[0].geometry.coordinates[1] "10"
  field-equals "$result" .features[0].geometry.type "Point"
  field-exists "$result" .features[0].lastUpdated
}
