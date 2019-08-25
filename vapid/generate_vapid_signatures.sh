#!/usr/bin/env bash
die() {
    echo VAPID keys could not be generated.
    exit
}

trap 'die' ERR

PRIVATE_KEY_FILE=${PRIVATE_KEY_FILE:-private.pem}
PUBLIC_KEY_FILE=${PUBLIC_KEY_FILE:-public.pem}
PUBLIC_KEY_FILE_BASE64=${PUBLIC_KEY_FILE_BASE64:-public.base64}
PUBLIC_KEY_FILE_HEX_ARRAY=${PUBLIC_KEY_FILE_HEX_ARRAY:-public.js}

# Generate the VAPID keys according to https://pimeys.github.io/rust-web-push/v0.4.0/web_push/struct.VapidSignatureBuilder.html

# Generate the private key
openssl ecparam -name prime256v1 -genkey -noout -out $PRIVATE_KEY_FILE

# Generate the public key
openssl ec -in $PRIVATE_KEY_FILE -pubout -out $PUBLIC_KEY_FILE 2> /dev/null

# Generate the public key in base64 (for client output)
openssl ec -in $PRIVATE_KEY_FILE -pubout -outform DER 2> /dev/null | tail -c 65 | base64 | tr '/+' '_-' | tr -d '\n' > $PUBLIC_KEY_FILE_BASE64

# Generate the public key as hexadecimal bytes (for client output)
echo -n "Uint8Array([" > $PUBLIC_KEY_FILE_HEX_ARRAY
openssl ec -in $PRIVATE_KEY_FILE -pubout -outform DER 2> /dev/null | tail -c 65 | od -A n -t x1 | tr -d '\n' | sed 's/ /,0x/g' | tail -c +2 >> $PUBLIC_KEY_FILE_HEX_ARRAY
echo -n "])" >> $PUBLIC_KEY_FILE_HEX_ARRAY
