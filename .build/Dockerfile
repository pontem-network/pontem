FROM rust:1.49 as build

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang bash

SHELL ["/bin/bash", "-c"]

WORKDIR /opt/build
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
	make build && \
	mkdir -p release && cp target/release/mv-node release/

FROM ubuntu:20.04
WORKDIR /opt/sp-move
ENV PATH="${PATH}:/opt/sp-move"
COPY --from=build /opt/build/release/* ./
