#!/usr/bin/env bats

load framework

# This test ensures that
#   1. images can be uploaded
#   2. images can be downloaded
#   3. images are unmodified after uploading and downloading them
#   4. the url property of images gets properly set
#   5. the id identifies the image properly
@test "Image upload" {
  # add facility
  create-facility "Foobar" 10 11

  # get the source id and original id for the facility
  local result=$(request get facilities/by-radius/11/10/1)
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"

  local sourceId=$(extract-field "$result" .features[0].properties.sourceId)
  local originalId=$(extract-field "$result" .features[0].properties.originalId)

  # generate a random image
  local width=1024
  local height=1024
  local tmpdir=$(mktemp -d)
  head -c "$((3*width*height))" /dev/urandom | convert -depth 8 -size "$width"x"$height" RGB:- "$tmpdir/image.jpg"

  local result=$(request post-multipart "/images/upload/$sourceId/$originalId?lat=10&lon=11" image=@"$tmpdir/image.jpg;type=image/jpeg")
  field-equals "$result" '.results | length' "1"
  field-equals "$result" .results[0].result "success"

  local imageId=$(extract-field "$result" .results[0].id)

  local result=$(request get "facilities/by-id/$sourceId/$originalId")
  field-equals "$result" .result "success"
  field-equals "$result" .featureCount "1"
  field-equals "$result" '.features[0].properties.images | length' "1"
  field-equals "$result" .features[0].properties.images[0].id "$imageId"
  field-exists "$result" .features[0].properties.images[0].url

  local url=$(extract-field "$result" .features[0].properties.images[0].url)
  diff <(echo "$url") <(echo "https://tonari.app/api/images/$imageId")

  diff <(request get "/images/$imageId") "$tmpdir/image.jpg"

  rm -r "$tmpdir"
}
