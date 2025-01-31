
## Pull the RBE image

docker pull --platform=linux/amd64 ghcr.io/marvin-hansen/rbe-custom/rbe:latest

## Start the docker image

docker run -d -i --name=bb_x86 --platform=linux/amd64 ghcr.io/marvin-hansen/rbe-custom/rbe

## Connect to the docker image

docker exec -it bb_x86 bash

## Install basics

apt-get install make

## Install GH CLI 

(type -p wget >/dev/null || (  apt update &&   apt-get install wget -y)) \
&&   mkdir -p -m 755 /etc/apt/keyrings \
&& wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg |   tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
&&   chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" |   tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
&&   apt update \
&&   apt install gh -y

apt update
apt install gh

gh auth login

## Install bazelisk

 wget https://github.com/bazelbuild/bazelisk/releases/download/v1.8.1/bazelisk-linux-amd64

 chmod +x bazelisk-linux-amd64

mv bazelisk-linux-amd64 /usr/local/bin/bazel

## Fast clone the repo:

git clone --depth=1 https://github.com/marvin-hansen/quant-engine/

# make sure you get the binary available in $PATH
 which bazel


## Build

cd quant-engine
make build 