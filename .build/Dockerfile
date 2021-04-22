FROM rust:1.49 as build

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang bash

SHELL ["/bin/bash", "-c"]

WORKDIR /opt/build
ARG DOVE_RELEASE=1.1.1
ARG DOVE_VERSION=1.1.0
# add dove
RUN curl -sL --fail -o "/usr/local/cargo/bin/dove" "https://github.com/pontem-network/move-tools/releases/download/${DOVE_RELEASE}/dove-${DOVE_VERSION}-linux-x86_64" && \
	chmod +x /usr/local/cargo/bin/dove && \
    dove -V
COPY ./scripts/ ./scripts/
COPY ./Makefile ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	--mount=type=cache,target=/opt/build/target \
	make init

COPY ./ ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	--mount=type=cache,target=/opt/build/target \
	rustup target add wasm32-unknown-unknown && \
	make test && make build && \
	mkdir -p release && \
	cp target/release/mv-node release/

FROM ubuntu:20.04
WORKDIR /opt/sp-move
ENV PATH="${PATH}:/opt/sp-move"
COPY --from=build /opt/build/release/* ./
