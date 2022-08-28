# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd sources/ghostdog/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    bash -c "pushd sources/parse-datetime/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv sources/ghostdog/fuzz/target/x86_64-unknown-linux-gnu/release/img_scanning /img_scanning && \
    mv sources/parse-datetime/fuzz/target/x86_64-unknown-linux-gnu/release/parse /parse && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd sources/ghostdog/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    bash -c "pushd sources/parse-datetime/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv sources/ghostdog/fuzz/target/release/img_scanning /img_scanning_no_inst && \
    mv sources/parse-datetime/fuzz/target/release/parse /parse_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /img_scanning /img_scanning_no_inst /
COPY --from=builder /parse /parse_no_inst /
