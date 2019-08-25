# API Documentation

This is the documentation for the API of the backend. The purpose of this API is to act as an overlay to
the data from the [accessibility.cloud](https://www.accessibility.cloud/).

## Table of Contents

- [Connection to the accessibility.cloud](#connection-to-the-accessibility.cloud)
- [Invariants](#invariants)
- [Requesting Facility Data](#requesting-facility-data)
  - [Format](#format)
  - [Retrieve Facilities Within a Map Tile](#retrieve-facilities-within-a-map-tile-facilitiesby-tilexyz)
  - [Retrieve Facilities by Radius](#retrieve-facilities-by-radius-facilitiesby-radiuslongitudelatituderadius`)
  - [Retrieve a Facility by ID](#retrieve-a-facility-by-id-facilitiesby-idsourceIdoriginalId)
  - [Retrieve all Facilities from a Single Source](#retrieve-all-facilities-from-a-single-source-facilitiesby-source-idsourceid)
  - [Retrieve Facilities Updated Since the Specified Date](#retrieve-facilities-updated-since-the-specified-date-facilitiesupdated-sincetimestampsourceidsourceid)
  - [Retrieve an Image](#retrieve-an-image-imagesid)
- [Changing Facility Data](#changing-facility-data)
  - [Example API Request Code](#example-api-request-code)
  - [Note: Adding New Facilities](#note-adding-new-facilities)
  - [Create or Update a Facility](#create-or-update-a-facility-facilitiesset-facility)
  - [Add a Comment to a Facility](#add-a-comment-to-a-facility-facilitiesadd-comment)
  - [Flag a Comment as Inappropriate](#flag-a-comment-as-inappropriate-facilitiesflag-comment)
  - [Verify Attributes of a Facility](#verify-attributes-of-a-facility-facilitiesverify-attributes)
  - [Indicate That a User Wishes to Visit a Facility](#indicate-that-a-user-wishes-to-visit-a-facility-facilitieswill-visit)
    - [Result](#result)
  - [Upload an Image](#upload-an-image-imagesuploadsourceidoriginalidlatlatlonlon)
    - [Note](#note)
    - [Example](#example)
  - [Label an Image](#label-an-image-imagesset-label)
  - [Verify a Label for an Image](#verify-a-label-for-an-image-imagesverify-label)
  - [Flag an Image](#flag-an-image-imagesflag-image)

## Connection to the accessibility.cloud

The `sourceId` in the results corresponds to the `sourceId` in the accessibility.cloud data. The `originalId`
corresponds to the ID in the original source. Note that we assume the tuple `(sourceId, originalId)` to be
unique. Thus we use the tuple as an ID for facilities.

Please note that the data from this API is only complete when combined with the data from the accessibility
cloud. The database only stores changes to the data and not a whole copy.

## Invariants

A facility cannot exist without the following data in the database.

- `"geometry"`: Within it the location is specified in `"coordinates"`. Note that the format of `"coordinates"`
  is `[longitude, latitude]`. This is needed so that facilities can be searched by using their geographic location.
- `"lastUpdated"`: Contains the time and date at which the facility entry in the database was last updated.
  This is needed, so that incremental updates of other copies of the data can be easily supported.
- `"properties"`: Contains various information about the facility. The following properties exist for every facility
  in the database:
  - `"sourceId"`: The ID of the source the facility originally came from. The ID is the same as the `"sourceID"`
    in the accessibility.cloud. This (in combination with the `"originalId"`) is needed to uniquely identify
    a facility even across database boundaries.
  - `"originalId"`: The ID of the facility in the source it originally came from. This (in combination with the
    `"sourceId"`) is needed to uniquely identify a facility even across database boundaries.

## Requesting Facility Data

This section describes all the methods of requesting data from the API.

### Format

The data is returned in JSON format. All request methods have the same result format. The `"result"`
of the returned object indicates the success of the operation. If the operation was successful,
the result is `"success"`. A successful operation contains two more attributes in the root object:

- an array named `"features"` is returned in the result, containing all the features
- a number named `"featureCount"`, which indicates the length of the features array

The following is an example result of the API.

```text
{
    "result": "success",
    "features": [
        {
            "type": "Feature",
            "properties": {
                "originalId": "0123456789",
                "sourceId": "A1B2C3D4E5F6",
                "category": "toilets",
                "images": [
                    {
                        "id": "12345678-90ab-cdef-1234-567890abcdef",
                        "url": "https://your.domain/images/12345678-90ab-cdef-1234-567890abcdef",
                        "label": "toilet"
                    }
                ],
                "name": "Example facility",
                "address": {
                    "street": "Example street",
                    "number": 123,
                    "city": "Example city",
                    "zip": 12345
                },
                "accessibility": {
                    "accessibleWith": {
                        "wheelchair": true
                    }
                },
                "comments": [
                    {
                        "content": "We won't survive the next second!",
                        "timestamp": "1999-12-31 23:59:59.578783 UTC"
                    }
                ],
                "_id": "1234567890abcdef12345678"
            },
            "geometry": {
                "coordinates": [
                    12.3456789,
                    12.3456789
                ],
                "type": "Point"
            },
            "lastUpdated": "1999-12-31 23:59:59.578783 UTC"
        },
    ],
    "featureCount": 1
}
```

### Retrieve Facilities Within a Map Tile (`/facilities/by-tile/<x>/<y>/<z>`)

Returns all facilities within the specified map tile which is given in the [slippy map tile format](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames).
`x` and `y` are coordinates of the tile, while `z` corresponds to the zoom level.

### Retrieve Facilities by Radius (`/facilities/by-radius/<longitude>/<latitude>/<radius>`)

Returns all facilities within the `radius` around the point located at `longitude` and `latitude`.
The `radius` is given in meters.

The facilities returned by this query have an additional `"property"` called `"distance"` which
contains the distance to the point located at `longitude` and `latitude`.

### Retrieve a Facility by ID (`/facilities/by-id/<sourceId>/<originalId>`)

Returns the facility with the specified `sourceId` and `originalId`.
Please note that the return format is the same, so the `"features"` array is either empty or
contains exactly one element.

### Retrieve all Facilities from a Single Source (`/facilities/by-source-id/<sourceId>`)

Returns all facilities with the specified `sourceId`.

### Retrieve Facilities Updated Since the Specified Date (`/facilities/updated-since/<timestamp>?sourceId=<sourceId>`)

Returns all facilities that have been updated since the specified `timestamp` (in the format `yyyy-MM-dd HH:mm:ss.SSS`
where `yyyy` is the year ,`MM` is the month, `dd` is the day, `HH` is the hour in 24 hour format, `mm` are the minutes,
`ss` are the seconds and `SSS` are the fractional seconds). Optionally you may specify a `sourceId` to only ask for
facilities from the specified source.

### Retrieve an Image (`/images/<id>`)

Returns the image with the specified `id`. Note that the result is not JSON, but rather a JPEG image.

## Changing Facility Data

Requests to change facility data are made in JSON format (except for the image upload).
The result is a JSON object with an attribute `"result"`, which will have the value
`"success"` if the operation was successful.
If `"result"` is not `"success"`, there may optionally be an attribute
`"reason"`, which is a description of the reason the result was not successful.
If no such attribute is present, the error is most likely a database error.

All requests in this section use the HTTP POST method.

### Example API Request Code

An API request could look the following way:

```javascript
    var comment_content = document.getElementById("comment_content").value;
    var request_data = {
        lat: 12.345678,
        lon: 12.345678,
        id: {
            sourceId: "A1B2C3D4E5F6",
            originalId: "0123456789"
        },
        content: comment_content
    };
    fetch("https://your.domain/facilities/add-comment", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(request_data)
    })
    .then(response => response.json())
    .catch(error => console.log(error))
    .then(response => {
        if(response.result === "success") {
            console.log("Request was successful");
        } else {
            console.log("Request was not successful");
        }
    });
```

### Note: Adding New Facilities

Adding new facilities to the database can happen in two ways:

- **If the facility already exists in the accessibility.cloud**: In this case the facility is
  automatically added to the database. In order to make this possible, the location and the
  ID tuple have to be present in every API-request that could add a new facility.
- **If the facility does not yet exist in the accessibility.cloud (and therefore does not yet
  have a `sourceId` and `originalId`)**: In this case the process of adding the facility is
  referred to as "creating a new facility". This has to made explicit. Currently the only way
  of creating a new facility is to use the `/facilities/set-facility` API.

### Create or Update a Facility (`/facilities/set-facility`)

Sets some of the data associated with the facility. This request may either create a new facility or
update an existing one. When updating facility data, a deep update is performed, this means, that
when setting for example `"address": { "street": "Example street" }`, all siblings of `"street"`
in `"address"` will remain untouched and only the value of `"address.street"` will be updated.

#### Format

```text
{
    "createNewFacility": Bool,
    "id": {
        "sourceId": String,
        "originalId": String,
    },
    "lat": Number,
    "lon": Number,
    "name": String,
    "address": Object,
    "accessibility": Object,
}
```

#### Parameters

- `"createNewFacility"`: This parameter is required. It controls whether a new facility is created (see
  the [section on "Adding New Facilities"](#note-adding-new-facilities)).
- `"id"`: This parameter is required if `"createNewFacility"` is `false`. It specifies the ID tuple of
  the existing facility.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"lat"`: This parameter is required. The latitude of the facility.
- `"lon"`: This parameter is required. The longitude of the facility.
- `"name"`: This parameter is optional. If present, the name of the facility is updated according to the
  value of this parameter.
- `"address"`: This parameter is optional. If present, the address of the facility is updated according to
  the value of this parameter.
- `"accessibility"`: This parameter is optional. If present, the accessibility of the facility is updated
  according to the value of this parameter.

### Add a Comment to a Facility (`/facilities/add-comment`)

Adds a comment to a facility. The comment consists of its content and a timestamp of when it was
posted.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "lat": Number,
    "lon": Number,
    "content": String,
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"lat"`: This parameter is required. The latitude of the facility.
- `"lon"`: This parameter is required. The longitude of the facility.
- `"content"`: This parameter is required. Its value is the content of the comment to add.

### Flag a Comment as Inappropriate (`/facilities/flag-comment`)

Flags a comment as inappropriate. A flagged comment will not be returned by API requests.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "commentId": String
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.

### Verify Attributes of a Facility (`/facilities/verify-attributes`)

Verifies attributes about a facility. Note that the attributes don't necessarily need to be stored
on the server at the time of verification, as they could be stored on other servers.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "attributes": Array<String>,
    "lat": Number,
    "lon": Number
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility for which the attributes
  are to be verified.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"attributes"`: This parameter is required. These are the attributes that should be verified.
- `"lat"`: This parameter is required. The latitude of the facility.
- `"lon"`: This parameter is required. The longitude of the facility.

### Indicate That a User Wishes to Visit a Facility (`/facilities/will-visit`)

Notifies the backend that a user wants to visit a facility. This can then be used by the server
to better assess the quality of a facility. If users frequently chose one facility, but not the
other, then the quality of the former is likely higher than that of the latter.

The server also returns questions about the facility that the user can answer.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "search": {
        "lat": Number,
        "lon": Number,
        "radius": Number
    }
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the visited facility.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"search"`: This parameter is required. It represents the radius-search that was used to find the
  facility.
  - `"lat"`: This parameter is required. The latitude of the search origin.
  - `"lon"`: This parameter is required. The longitude of the search origin.
  - `"radius"`: This parameter is required. The radius of the search.

#### Result

If the `"result"` in the returned JSON is `"success"`, then there exists an array named `"questions"`
in the returned JSON. This is an array of question objects, which take the following form.

```text
{
    "type": String,
    "attribute": String,
    "value": Any,
    "imageURL": String,
    "imageLabel": String
}
```

- `"type"`: This is the only field that is always present. It specifies the type of question. One of
  `"addImage"`, `"labelImage"`, `"verifyLabel"`, `"askAttribute"`, `"verifyAttribute"`.
- `"attribute"`: This field is only present if the `"type"` is `"askAttribute"` or `"verifyAttribute"`.
  It specifies the attribute that the question is about.
- `"value"`: This field is only present if the `"type"` is `"verifyAttribute"` and specifies the value
  of the attribute that is to be verified.
- `"imageURL"`: This field is only present if the `"type"` is `"labelImage"` or `"verifyLabel"`. It
  represents the URL of the image that the question is about.
- `"imageLabel"`: This field is only present if the `"type"` is `"addImage"` or `"verifyLabel"`. If the
  `"type"` is `"addImage"`, it represents the type of image that is requested. If the type is `"verifyLabel"`,
  it represents the label that should be verified.
  Allowed values are `"toilet"` (for the toilet itself), `"entry"` (for the entry from the outside),
  `"sink"` (for the sink), `"fromEntry"` (for a picture from the entry of the room) and `"other"`
  (for anything else related that's important).

### Upload an Image (`/images/upload/<sourceId>/<originalId>?lat=<lat>&lon=<lon>`)

Uploads the specified images to the server and adds them to the facility with the given `sourceId`
and `originalId`. `lat` and `lon` refer to the latitude and longitude of the facility.

The images itself must have the name `image` in the form used to post this to the server. Multiple
images may be uploaded at once.

#### Note

This is the only update API that does not use `application/json` as a content type. It uses
`multipart/form-data` instead to handle uploading the images better.

If the facility doesn't exist in the database, we have to create a new entry for which we need its location in order to hold the invariant that each entry must have a location associated to it. Since we cannot know if the facility exists in the database beforehand, the location must always be provided.

Currently only JPEG images can be uploaded.

#### Example

```html
<form method="post" action="../images/upload/A1B2C3D4E5F6/0123456789?lat=12.345678&lon=12.345678" enctype="multipart/form-data">
    <input type="file" name="image" accept="image/jpeg" multiple/>
    <input type="submit"/>
</form>
```

### Label an Image (`/images/set-label`)

Sets the label on an image. This corresponds to the `"label"` property in the JSON representation
of the image.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "imageURL": String,
    "imageLabel": String,
    "lat": Number,
    "lon": Number
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility to which the
  image belongs.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"imageURL"`: This parameter is required. It specifies the URL of the image to label. Note that this
  may be a URL on a different server. In this case the image is labeled and added to the facility. If
  the facility does not yet exist, it is created.
- `"imageLabel"`: This parameter is required. It specifies the label to set for the image.
- `"lat"`: This parameter is required. The latitude of the facility.
- `"lon"`: This parameter is required. The longitude of the facility.

### Verify a Label for an Image (`/images/verify-label`)

Verifies the label for an image. Subsequent API requests will mark the image as verified.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "imageURL": String,
    "imageLabel": String
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility to which the
  image belongs.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"imageURL"`: This parameter is required. The URL of the image the label belongs to. The image must exist
  within the database in order for the label verification to work.
- `"imageLabel"`: This parameter is required. It contains the label of the image that is verified.
  This ensures that a user cannot verify a label that has changed in the database in the meantime (i.e. it
  prevents race conditions).

### Flag an Image (`/images/flag-image`)

Flags an image as inappropriate. A flagged image will not be returned by API requests.

#### Format

```text
{
    "id": {
        "sourceId": String,
        "originalId": String
    },
    "imageURL": String
}
```

#### Parameters

- `"id"`: This parameter is required. It specifies the ID tuple of the facility to which the
  image belongs.
  - `"sourceId"`: This parameter is required. The ID of the source in the accessibility.cloud.
  - `"originalId"`: This parameter is required. The ID of the facility in the original source.
- `"imageURL"`: This parameter is required. It specifies the URL of the image to flag.
