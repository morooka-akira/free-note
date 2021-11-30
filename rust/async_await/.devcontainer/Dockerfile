FROM rust:1.56.1

# it makes faster cargo build
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

# linter and formatter
RUN rustup component add clippy
RUN rustup component add rustfmt

# for VS Code Remote Container
RUN rustup component add rust-analysis
RUN rustup component add rust-src
RUN rustup component add rls
