# Test Utils

## Docker Utils

Docker utils help to create, re-use, or stop containers during tests. Requires 
a Docker installation with a working CLI. 

For an usage example, see [src_docker_bin](src/docker_bin/main.rs). 

## TestEnv

TestEnv util helps to setup and configure a test environment i.e. for CI that 
comprises of multiple containers and possible initialization steps i.e. install DB schema.

Requires a Docker installation and authentication to private image registries if private container images are used.

For an usage example, see [test env bin](src/test_env_bin/main.rs).