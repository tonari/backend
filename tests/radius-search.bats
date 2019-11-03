#!/usr/bin/env bats

load framework

# This test ensures that
#   1. radius search properly includes and excludes facilities
#   2. the order of the results is sorted by distance and that the distance is reported correctly
@test "Radius search" {
  # add three included facilities
  request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11,
    "name": "Foobar inside"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11.00001,
    "name": "Foobar inside 2"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11.0001,
    "name": "Foobar inside 3"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  # add a facility that should not be included
  request=$(cat <<JSON
{
    "createNewFacility": true,
    "lat": 10,
    "lon": 11.001,
    "name": "Foobar outside"
}
JSON
)
  expect post facilities/set-facility '{"result":"success"}' "$request"

  # test that facility does exist
  result=$(request get facilities/by-radius/11/10/100)

  # Radius search will only work if either the index is initialized or if there
  # are no entries. Otherwise it will fail.
  diff <(docker logs "$TONARI" 2>&1 | rg "^Error:|^Warning:|^thread '.*' panicked at") <(echo -n '')

  is-json "$result"
  field-equals "$result" .result success
  field-equals "$result" .featureCount 3
  field-equals "$result" '.features | length' 3
  field-equals "$result" .features[0].properties.name "Foobar inside"
  field-equals "$result" .features[0].properties.distance "0"
  field-equals "$result" .features[1].properties.name "Foobar inside 2"
  field-equals "$result" .features[1].properties.distance "1.096"
  field-equals "$result" .features[2].properties.name "Foobar inside 3"
  field-equals "$result" .features[2].properties.distance "10.964"
}
