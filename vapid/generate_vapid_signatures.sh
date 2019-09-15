#!/usr/bin/env bash
die() {
    echo VAPID keys could not be generated.
    exit
}

trap 'die' ERR

PRIVATE_KEY_FILE=${PRIVATE_KEY_FILE:-private.pem}
PUBLIC_KEY_FILE=${PUBLIC_KEY_FILE:-public.pem}
PUBLIC_KEY_FILE_HEX=${PUBLIC_KEY_FILE_BASE64:-public.hex}

# Generate the VAPID keys according to https://pimeys.github.io/rust-web-push/v0.4.0/web_push/struct.VapidSignatureBuilder.html

# Generate the private key
openssl ecparam -name prime256v1 -genkey -noout -out $PRIVATE_KEY_FILE

# Generate the public key
openssl ec -in $PRIVATE_KEY_FILE -pubout -out $PUBLIC_KEY_FILE 2> /dev/null

# Generate the public key in hex (for client output)
openssl ec -in $PRIVATE_KEY_FILE -pubout -outform DER 2> /dev/null | tail -c 65 | xxd -p | tr -d '\n' > $PUBLIC_KEY_FILE_HEX
