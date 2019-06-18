#!/usr/bin/env bash

set -x

cd
mkdir -p $HOME/local

PROTOC_URL=https://github.com/protocolbuffers/protobuf/releases/download/v3.8.0/protoc-3.8.0-linux-x86_64.zip

curl -fSsL ${PROTOC_URL} -o protoc.zip
unzip protoc.zip -d $HOME/local

find $HOME/local
