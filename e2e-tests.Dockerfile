FROM ubuntu:22.04

ARG BUILD_SPIN=false
ARG FETCH_SPIN=true
ARG SPIN_VERSION=canary

WORKDIR /root
RUN apt-get update && apt-get install -y wget sudo xz-utils gcc git pkg-config redis clang libicu-dev docker.io

# nodejs
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
RUN apt-get install -y nodejs npm

# golang
RUN wget https://go.dev/dl/go1.20.1.linux-amd64.tar.gz && \
    rm -rf /usr/local/go && tar -C /usr/local -xzf go1.20.1.linux-amd64.tar.gz
ENV PATH="$PATH:/usr/local/go/bin"


