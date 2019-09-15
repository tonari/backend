//! This module contains pages for testing purposes during development.

use rocket::{
    get,
    response::{content::Html, NamedFile},
    routes, Route,
};

/// The routes for all the different test pages.
pub(crate) fn testpage_routes() -> Vec<Route> {
    routes![
        testpage_index,
        testpage_image_upload,
        testpage_set_image_label,
        testpage_update_facility,
        testpage_icon,
        testpage_will_visit,
        testpage_add_comment,
        testpage_verify_attributes,
        testpage_flag_image,
        testpage_verify_image_label,
        testpage_flag_comment,
    ]
}

/// The main test page containing links to the other test pages.
#[get("/")]
fn testpage_index() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Test pages</title>
            </head>
            <body>
                <h1>Test pages</h1>
                <ul>
                    <li><a href="./imageupload">Image upload</a></li>
                    <li><a href="./set-image-label">Set image label</a></li>
                    <li><a href="./verify-image-label">Verify image label</a></li>
                    <li><a href="./flag-image">Flag images</a></li>
                    <li><a href="./set-facility">Set facility data</a></li>
                    <li><a href="./will-visit">Will visit a SF</a></li>
                    <li><a href="./add-comment">Add comments</a></li>
                    <li><a href="./flag-comment">Flag comments</a></li>
                    <li><a href="./verify-attributes">Verify attributes</a></li>
                </ul>
            </body>
         </html>"#,
    )
}

/// The test page for uploading images.
#[get("/imageupload")]
fn testpage_image_upload() -> Html<&'static str> {
    Html(r#"
        <html>
            <head>
                <title>Image upload</title>
            </head>
            <body>
                <h1>Image upload</h1>
                <form method="post" action="../images/upload/TEST_SOURCE_ID/TEST_ORIGINAL_ID?lat=52.526159&lon=13.400332" enctype="multipart/form-data">
                    <input type="file" name="image" accept="image/jpeg" multiple/>
                    <input type="submit"/>
                </form>
            </body>
        </html>"#
    )
}

/// The test page for setting an image type.
#[get("/set-image-label")]
fn testpage_set_image_label() -> Html<&'static str> {
    Html(r#"
        <html>
            <head>
                <title>Set image label</title>
                <script type="text/javascript">
                    function submit() {
                        var image_url = document.getElementById("image_url").value;
                        var image_label = document.getElementById("image_label").value;
                        var doc = {
                            imageURL: image_url,
                            imageLabel: image_label,
                            lat: 52.526159,
                            lon: 13.400332,
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            }
                        };
                        let request = new XMLHttpRequest();
                        request.open("POST", "../images/set-label", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.onreadystatechange = function (e) {
                            document.getElementById("result").innerHTML = JSON.stringify(JSON.parse(request.responseText), null, 2);
                        };
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Set image label</h1>
                <label for="image_url">Image-URL:</label>
                <input type="text" name="image_url" id="image_url"/>
                <label for="image_label">Label:</label>
                <input type="text" name="image_label" id="image_label"/>
                <input type="button" onclick="submit()" value="Submit"/>
                <pre style="background: #ddd;" id="result"></pre>
            </body>
        </html>"#
    )
}

/// The test page for setting an image type.
#[get("/verify-image-label")]
fn testpage_verify_image_label() -> Html<&'static str> {
    Html(r#"
        <html>
            <head>
                <title>Verify image label</title>
                <script type="text/javascript">
                    function submit() {
                        var image_url = document.getElementById("image_url").value;
                        var image_label = document.getElementById("image_label").value;
                        var doc = {
                            imageURL: image_url,
                            imageLabel: image_label,
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            }
                        };
                        let request = new XMLHttpRequest();
                        request.open("POST", "../images/verify-label", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.onreadystatechange = function (e) {
                            document.getElementById("result").innerHTML = JSON.stringify(JSON.parse(request.responseText), null, 2);
                        };
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Verify image label</h1>
                <label for="image_url">Image-URL:</label>
                <input type="text" name="image_url" id="image_url"/>
                <label for="image_label">Image-Label:</label>
                <input type="text" name="image_label" id="image_label"/>
                <input type="button" onclick="submit()" value="Submit"/>
                <pre style="background: #ddd;" id="result"></pre>
            </body>
        </html>"#
    )
}

/// The test page for flagging an image.
#[get("/flag-image")]
fn testpage_flag_image() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Flag image</title>
                <script type="text/javascript">
                    function submit() {
                        var image_url = document.getElementById("image_url").value;
                        var doc = {
                            imageURL: image_url,
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            }
                        };
                        let request = new XMLHttpRequest();
                        request.open("POST", "../images/flag-image", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Flag image</h1>
                <label for="image_url">Image-URL:</label>
                <input type="text" name="image_url" id="image_url"/>
                <input type="button" onclick="submit()" value="Submit"/>
            </body>
        </html>"#,
    )
}

/// Sets or updates the facility.
#[get("/set-facility")]
fn testpage_update_facility() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Set facility data</title>
                <script type="text/javascript">
                    function submit(add_new) {
                        var name = document.getElementById("name").value;
                        var address = document.getElementById("address").value;
                        var accessibility = document.getElementById("accessibility").value;
                        var doc = {
                            lat: 52.526159,
                            lon: 13.400332,
                            createNewFacility: true
                        };
                        if(name.length && name.length !== 0) {
                            doc.name = name;
                        }
                        if(address.length && address.length !== 0) {
                            try {
                                doc.address = JSON.parse(address);
                            } catch { /* invalid JSON */ }
                        }
                        if(accessibility.length && accessibility.length !== 0) {
                            try {
                                doc.accessibility = JSON.parse(accessibility);
                            } catch { /* invalid JSON */ }
                        }
                        if(!add_new) {
                            doc.createNewFacility = false;
                            doc.id = {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            };
                        }
                        let request = new XMLHttpRequest();
                        request.open("POST", "../facilities/set-facility", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Set facility data</h1>
                <label for="name">Name:</label>
                <input type="text" id="name" name="name"/>
                <label for="address">Address:</label>
                <input type="text" id="address" name="address"/>
                <label for="accessibility">Accessiblity:</label>
                <input type="text" id="accessibility" name="accessibility"/>
                <input type="button" onclick="submit(true)" value="Add new"/>
                <input type="button" onclick="submit(false)" value="Add to existing"/>
            </body>
        </html>"#,
    )
}

/// Sends a test page to test adding comments.
#[get("/add-comment")]
fn testpage_add_comment() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Add a comment</title>
                <script type="text/javascript">
                    function submit() {
                        var content = document.getElementById("content").value;
                        var doc = {
                            lat: 52.526159,
                            lon: 13.400332,
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            }
                        };
                        if(content !== "") {
                            doc.content = content;
                        }
                        let request = new XMLHttpRequest();
                        request.open("POST", "../facilities/add-comment", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Add a comment</h1>
                <label for="content">Content:</label>
                <input type="text" id="content" name="content"/>
                <input type="button" onclick="submit()" value="submit"/>
            </body>
        </html>"#,
    )
}

/// Sends a test page to test flagging comments.
#[get("/flag-comment")]
fn testpage_flag_comment() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Flag a comment</title>
                <script type="text/javascript">
                    function submit() {
                        var id = document.getElementById("id").value;
                        var doc = {
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            },
                            commentId: id
                        };
                        let request = new XMLHttpRequest();
                        request.open("POST", "../facilities/flag-comment", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Flag a comment</h1>
                <label for="id">ID:</label>
                <input type="text" id="id" name="id"/>
                <input type="button" onclick="submit()" value="submit"/>
            </body>
        </html>"#,
    )
}

/// Sends a test page to test verifying attributes.
#[get("/verify-attributes")]
fn testpage_verify_attributes() -> Html<&'static str> {
    Html(r#"
        <html>
            <head>
                <title>Verify attributes</title>
                <script type="text/javascript">
                    function submit() {
                        var attributes = document.getElementById("attributes").value;
                        var doc = {
                            lat: 52.526159,
                            lon: 13.400332,
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: "TEST_ORIGINAL_ID"
                            }
                        };
                        if(attributes !== "") {
                            doc.attributes = JSON.parse(attributes);
                        }
                        let request = new XMLHttpRequest();
                        request.open("POST", "../facilities/verify-attributes", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        let data = JSON.stringify(doc);
                        request.onreadystatechange = function (e) {
                            document.getElementById("result").innerHTML = JSON.stringify(JSON.parse(request.responseText), null, 2);
                        };
                        request.send(data);
                    }
                </script>
            </head>
            <body>
                <h1>Verify attributes</h1>
                <label for="attributes">Attributes:</label>
                <input type="text" id="attributes" name="attributes"/>
                <input type="button" onclick="submit()" value="submit"/>
                <pre style="background: #ddd;" id="result"></pre>
            </body>
        </html>"#
    )
}

/// Serves the icon.
#[get("/icon.png")]
fn testpage_icon() -> Option<NamedFile> {
    NamedFile::open("testpages/icon.png").ok()
}

/// Serves the page that indicates that a user wants to visit a facility.
#[get("/will-visit")]
fn testpage_will_visit() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <script>
                    function willVisit() {
                        let id = document.getElementById("facilityId").value;

                        if (!id) {
                            id = "TEST_ORIGINAL_ID";
                        }

                        let request_data = {
                            id: {
                                sourceId: "TEST_SOURCE_ID",
                                originalId: id
                            },
                            search: {
                                lat: 52.526159,
                                lon: 13.400332,
                                radius: 1000
                            }
                        };

                        let request = new XMLHttpRequest();
                        request.open("POST", "../facilities/will-visit", true);
                        request.setRequestHeader("Content-Type", "application/json");
                        request.onreadystatechange = function (e) {
                            document.getElementById("result").innerHTML = JSON.stringify(JSON.parse(request.responseText), null, 2);
                        };
                        request.send(JSON.stringify(request_data));
                    }
                </script>
            </head>
            <body>
                <h1>Will visit</h1>
                <label for="facilityId">Facility ID:</label>
                <input type="text" id="facilityId" name="facilityId"/>
                <button onclick="willVisit()">Will visit a sanitary facility</button>
                <pre style="background: #ddd;" id="result">
                </pre>
            </body>
        </html>
    "#)
}
