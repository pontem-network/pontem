ARG POLKADOT_VERSION
FROM parity/polkadot:${POLKADOT_VERSION} as polkadot
FROM ubuntu:20.04

# Install deps
RUN apt update && apt install -y curl
ARG NODE_VERSION
RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash -
RUN apt install -y nodejs
RUN npm install polkadot-launch -g

# Add polkadot
WORKDIR /opt/polkadot/target/release
COPY --from=polkadot /usr/bin/polkadot ./
WORKDIR /opt/pontem/target/release

# Add pontem
ARG PONTEM_VERSION
RUN curl -#kSLf -o "pontem" "https://github.com/pontem-network/pontem/releases/download/${PONTEM_VERSION}/pontem-ubuntu-x86_64" && \
	chmod +x pontem
ENV PATH="${PATH}:/opt/polkadot/target/release:/opt/pontem/target/release"

WORKDIR /opt/app
SHELL ["/bin/bash", "-c"]

# add nimbus key
RUN mkdir -p ~/.pontem/keystore-1 && \
	pontem key insert --suri "//Alice" --keystore-path ~/.pontem/keystore-1 --key-type nmbs

CMD ["polkadot-launch", "/opt/pontem/launch-config.json"]
