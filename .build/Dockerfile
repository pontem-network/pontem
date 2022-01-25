FROM ubuntu:20.04 as build

ENV TZ=Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang bash build-essential libc6 libc-bin curl

SHELL ["/bin/bash", "-c"]
ENV PATH="${PATH}:/root/.cargo/bin"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN rustup update nightly && \
	rustup update stable

# add dove
ARG DOVE_VERSION=1.5.5
RUN curl -sL --fail -o "/usr/local/bin/dove" "https://github.com/pontem-network/move-tools/releases/download/${DOVE_VERSION}/dove-${DOVE_VERSION}-linux-x86_64" && \
	chmod +x /usr/local/bin/dove && \
    dove -V
WORKDIR /opt/build
COPY ./scripts/ ./scripts/
COPY ./Makefile ./
RUN make init

COPY ./ ./
RUN cargo clean -p pontem-node
RUN rustup target add wasm32-unknown-unknown && \
	make test && make build && \
	mkdir -p release && \
	cp target/release/pontem release/

FROM ubuntu:20.04
WORKDIR /opt/pontem
ENV PATH="${PATH}:/opt/pontem"
COPY --from=build /opt/build/release/* ./
