# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /lopdf/

WORKDIR /lopdf/fuzz/
RUN cargo fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>
COPY --from=builder /lopdf/fuzz/target/x86_64-unknown-linux-gnu/release/load_modify /
COPY --from=builder /lopdf/fuzz/target/x86_64-unknown-linux-gnu/release/stream /
COPY --from=builder /lopdf/fuzz/example.pdf /
