FROM ubuntu:20.04 as build

# RUN apk add curl git clang
ENV TZ=Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang bash build-essential libc6 libc-bin curl

ENV PATH="${PATH}:/root/.cargo/bin"
RUN curl -o "/tmp/rust.sh" --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs && \
	chmod +x /tmp/rust.sh && \
	/tmp/rust.sh -y
RUN rustup update nightly && \
	rustup update stable

SHELL ["/bin/bash", "-c"]

WORKDIR /opt/build
ARG DOVE_RELEASE=1.2.8
ARG DOVE_VERSION=1.2.8
# add dove
RUN curl -sL --fail -o "/usr/local/bin/dove" "https://github.com/pontem-network/move-tools/releases/download/${DOVE_RELEASE}/dove-${DOVE_VERSION}-linux-x86_64" && \
	chmod +x /usr/local/bin/dove && \
    dove -V
COPY ./scripts/ ./scripts/
COPY ./Makefile ./
RUN cargo clean -p mv-node
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
