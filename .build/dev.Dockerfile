FROM ubuntu:20.04 as build

ENV TZ=Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y make cmake pkg-config libssl-dev git clang bash build-essential libc6 libc-bin curl jq

SHELL ["/bin/bash", "-c"]
ENV PATH="${PATH}:/root/.cargo/bin"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# RUN export DOVE_LATEST="$(curl -s https://api.github.com/repos/pontem-network/move-tools/releases/latest | jq -r '.name')" && \
# 	curl -sL --fail -o "/usr/local/bin/dove" "https://github.com/pontem-network/move-tools/releases/download/${DOVE_LATEST}/dove-${DOVE_LATEST}-linux-x86_64" && \
# 	chmod +x /usr/local/bin/dove && \
#     dove -V

# add dove
ARG DOVE_VERSION=1.5.5
RUN curl -sL --fail -o "/usr/local/bin/dove" "https://github.com/pontem-network/move-tools/releases/download/${DOVE_VERSION}/dove-${DOVE_VERSION}-linux-x86_64" && \
	chmod +x /usr/local/bin/dove && \
    dove -V

WORKDIR /opt/pontem
