#!/usr/bin/env bats

load framework

# This test ensures that
#   1. radius search properly includes and excludes facilities
#   2. the order of the results is sorted by distance and that the distance is reported correctly
@test "Radius search" {
  # add three included facilities
  create-facility "Foobar inside" 10 11
  create-facility "Foobar inside 2" 10 11.000244140625
  create-facility "Foobar inside 3" 10 11.00048828125

  # add a facility that should not be included
  create-facility "Foobar outside" 10 11.0009765625

  # test that facility does exist
  local result=$(request get facilities/by-radius/11/10/100)

  # Radius search will only work if either the index is initialized or if there
  # are no entries. Otherwise it will fail.
  diff <(docker logs "$TONARI" 2>&1 | grep "^Error:|^Warning:|^thread '.*' panicked at") <(echo -n '')

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "3"
  field-equals "$result" '.features | length' "3"
  field-equals "$result" .features[0].properties.name "Foobar inside"
  field-equals "$result" .features[0].properties.distance "0"
  field-equals "$result" .features[1].properties.name "Foobar inside 2"
  field-equals "$result" .features[1].properties.distance "26.767"
  field-equals "$result" .features[2].properties.name "Foobar inside 3"
  field-equals "$result" .features[2].properties.distance "53.535"
}
