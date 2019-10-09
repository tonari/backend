FROM ekidd/rust-musl-builder:nightly-2019-08-13-openssl11 AS builder
ADD --chown=rust:rust . /home/rust/src
RUN cargo build --release && \
    strip /home/rust/src/target/x86_64-unknown-linux-musl/release/backend

FROM scratch
COPY --from=builder \
     /home/rust/src/target/x86_64-unknown-linux-musl/release/backend \
     /
ENTRYPOINT [ "/backend" ]
