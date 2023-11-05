# Memgraaph CLI

## Requirements

1) Docker
2) Rust
3) Make & Bash

## Create new memgraaph container

docker run -it -p 7687:7687 -p 7444:7444 -p 3000:3000 --name memgraph memgraph/memgraph-platform

## Start / stop memgraaph

docker start memgraph

docker stop memgraph

## web console

http://localhost:3000/

## Run test CLI

make run

or

cargo run --bin memgraph

## Ressources

https://memgraph.com/docs/client-libraries/rust