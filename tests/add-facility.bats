#!/usr/bin/env bats

load framework

# This test ensures two things:
# 1. Test if a facility can be retrieved after adding it
# 2. Test if the backend starts without any errors or warnings. This also tests
#    if the connection to the database works.
@test "Add facility" {
  # test that facility does not exist
  expect get facilities/by-radius/11/10/1 '{"result":"success", "featureCount": 0, "features": []}'

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

  # test that facility does exist
  result=$(request get facilities/by-radius/11/10/1)

  # Radius search will only work if either the index is initialized or if there
  # are no entries. Otherwise it will fail.
  diff <(docker logs "$TONARI" 2>&1 | rg "^Error:|^Warning:|^thread '.*' panicked at") <(echo -n '')

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"
  field-equals "$result" '.features | length' "1"
  field-equals "$result" .features[0].type "Feature"
  field-exists "$result" .features[0].properties.originalId
  field-exists "$result" .features[0].properties.sourceId
  field-equals "$result" .features[0].properties.category "toilets"
  field-equals "$result" .features[0].properties.name "Foobar"
  field-equals "$result" .features[0].geometry.coordinates[0] "11"
  field-equals "$result" .features[0].geometry.coordinates[1] "10"
  field-equals "$result" .features[0].geometry.type "Point"
  field-exists "$result" .features[0].lastUpdated
}
