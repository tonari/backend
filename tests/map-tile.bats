#!/usr/bin/env bats

load framework

# This test ensures that map tile queries return results only in their tile
@test "Map tile" {
  expect get facilities/by-tile/16/15/5 '{"result":"success", "featureCount": 0, "features": []}'

  # add two facilities
  create-facility "Foobar" 10 11
  create-facility "Foobar 2" 10 12

  # test that only one facility is returned
  local result=$(request get facilities/by-tile/16/15/5)

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"
  field-equals "$result" '.features | length' "1"
  field-equals "$result" .features[0].properties.name "Foobar"
  field-equals "$result" .features[0].geometry.coordinates[0] "11"
  field-equals "$result" .features[0].geometry.coordinates[1] "10"

  # test with a bigger tile
  local result=$(request get facilities/by-tile/8/7/4)

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "2"
  field-equals "$result" '.features | length' "2"

  # test with the biggest tile
  local result=$(request get facilities/by-tile/0/0/0)

  is-json "$result"
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "2"
  field-equals "$result" '.features | length' "2"
}
