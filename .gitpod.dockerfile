FROM rust:latest

# Install mob.sh tool
RUN curl -sL install.mob.sh | sh

# Ensure we have the latest stable channel.
RUN rustup install stable

# Install some rustup components that are available by default.
RUN rustup component add rustfmt clippy rust-src rust-docs rust-analysis rls llvm-tools-preview
