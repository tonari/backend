# Tonari backend

Tonari (from the Japanese 隣 which means "next door" / "the closest to you") is a webapp and API that enables searching for wheelchair-accessible sanitary facilities and enhancing the related datasets.

Check the live version out: https://tonari.app

This is the backend of Tonari.
Its purpose is to serve as a database for changes to facilities made in Tonari.

## Compiling

To compile this you will need a nightly Rust compiler, which you can find [here](https://rustup.rs/). The last tested version of the compiler is `rustc 1.33.0-nightly (a7be40c65 2018-12-26)`.

Then to compile the backend for debugging, run:

```bash
cargo build --features testpages
```

If you want to compile a release build, run:

```bash
cargo build --release
```

Alternatively you check out the `Makefile` to see what else you can do.
For example to open the documentation, you can run `make doc_open`, if you have `make` installed.

## Running

To start the backend, run the executable located at `target/release/backend` (replace `release` with `debug` for debug builds).

To run it in a staging or production enviroment, set the environment variable `ROCKET_ENV` to `staging` or `production`.

```bash
export ROCKET_ENV=production
```

## Configuration

To find out what configuration options are available, take a look at the configuration module (`src/configuration.rs`).

You can override the default values using environment variables of the same name prefixed with "`TONARI_`".
This is required for some variables (see [Required Configuration](#required-configuration)).

For example you could set the path where the images are stored in the following way:

```bash
export TONARI_IMAGE_PATH=/my/image/path
```

### Required Configuration

Before you can run this in production you need to do at least the following things:

- Set the environment variable `TONARI_SOURCE_ID` to the `sourceId` of Tonari in the accessibility.cloud.
- Set the environment variable `TONARI_IMAGE_URL_PREFIX` to the prefix of the URL where images are served.
  (see the section on [image urls](#image-urls)).
- Set the environment variable `TONARI_IMAGE_PATH` to the file path where the images are to be saved. Note
  that if you change this, images uploaded so far won't change their path, i.e. keep their old path.
- Set the `ROCKET_PORT` environment variable to the port you want to use.
- Set the `ROCKET_DATABASES` environment variable to contain the correct mongodb connection URL.
  For example:
  ```bash
  export ROCKET_DATABASES='{sanitary_facilities={url="mongodb://localhost:27017/sanitary_facilities"}}'
  ```
- Set the `ROCKET_SECRET_KEY` environment variable to a secret value that you need to generate.
  It should be the same value for each restart.
  This value is currently not used by the Tonari backend, but that may change in the future.
  To generate it, you can run `openssl rand -base64 32`.
  For example:
  ```bash
  [ ! -e /path/to/secret-key ] && openssl rand -base64 32 > /path/to/secret-key
  export ROCKET_SECRET_KEY=`cat /path/to/secret-key`
  ```

### Image URLs

In order to allow images to be flexible and also work with images on remote servers, the URLs for
images are generated at the time of their insertion into the database. However, this means that changing
the `TONARI_IMAGE_URL_PREFIX` variable will not update any existing image paths. That needs to be done manually
if necessary.

#### Serving Images Using a Different Server

Note that you may want to serve the images using a different server. If your server serves the images
at the URL `https://your.domain/my-images/` you can set the backend up so that any new image URLs are created
to point to that location. An example invocation of the server could look like this:

```bash
export TONARI_IMAGE_URL_PREFIX=https://your.domain/my-images/
export TONARI_IMAGE_URL_SUFFIX=.jpg
```

Note that the `TONARI_IMAGE_URL_SUFFIX` variable is set to the extension of the images. This allows URLs
like `https://your.domain/my-images/8efbfe48-9a8a-41a8-8b2d-307b8cfffff4.jpg` to be routed to your server.

## API Documentation

You can find the API documentation [here](API.md).

## Tests

Our integration tests have the following dependencies:

* Ubuntu:

  ```
  apt install bats docker.io openssl jq diffutils imagemagick ripgrep curl
  ```

* Arch Linux:

  ```
  pacman -S bash-bats docker jq diffutils imagemagick ripgrep
  ```

To run them use:

```
bats tests
```

## Interactively testing the API

Compile the backend with the `testpages` feature to have easy access to rudimentary testpages at `/testpages/`.

After you have installed MongoDB, you can use the `mongo` utility to view how the requests manipulate the data.

```bash
$ mongo
> use sanitary_facilities
> db.facilities.find().pretty()
```

Replace `sanitary_facilities` and `facilities` with the name you chose for the database and collection
respectively, if you used different names.
