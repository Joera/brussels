FROM fluencelabs/nox:0.23.6

# Lotus repository
ARG REPOSITORY="filecoin-project/lotus"
# Git branch of the lotus repository
ARG BRANCH="master"
# Filecoin network. Valid values: lotus(mainnet), calibnet
ARG NETWORK="lotus"

WORKDIR /opt

RUN apt-get update 
RUN apt-get install -y hwloc
RUN apt-get install -y mesa-opencl-icd ocl-icd-opencl-dev gcc git bzr jq pkg-config curl clang build-essential hwloc libhwloc-dev wget 

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# Download and install Go
ENV GO_VERSION=1.21.7
RUN wget https://golang.org/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz && \
    rm go${GO_VERSION}.linux-amd64.tar.gz
# Set Go environment variables
ENV PATH="/usr/local/go/bin:${PATH}"

# RUN sysctl -w net.core.rmem_max=2097152
# RUN sysctl -w net.core.rmem_default=2097152

# Install lotus
RUN git clone https://github.com/${REPOSITORY}.git --depth 1 --branch $BRANCH && \
    cd lotus && \
    make clean && \
    make deps && \
    make $NETWORK lotus-shed lotus-gateway && \
    install -C ./lotus /usr/local/bin/lotus && \
    install -C ./lotus-gateway /usr/local/bin/lotus-gateway && \
    install -C ./lotus-shed /usr/local/bin/lotus-shed

# how to get this in entrypoint.sh ????     
# RUN FULLNODE_API_INFO=wss://wss.node.glif.io/apigw/lotus nohup lotus daemon --lite > /dev/null 2>&1
# FULLNODE_API_INFO=wss://api.chain.love nohup lotus daemon --lite > /dev/null 2>&1

WORKDIR / 