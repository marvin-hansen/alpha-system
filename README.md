# Alpha System

Archived: Update dependencies before usage as the repo may contained outdated and unsecure depds. 

## About

A demo system for building multi-crate Rust systems with Bazel.

It showcases

* Cross compilation to Intel and Arm
* Proto / gRPC / Protobuf integration 
* End to end integration tests with Service and DockerUtils 
* Secure Container builds with Apko Images 
* Parallel container publishing with multi-run 

Integration tests are located in a dedicated crate next to each service under the folder alpha_system. The services are organized in groups i.e. core for all core services so under alpha_system/core/name you find a crate name_tests where the end to end integration tests are located.  

## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work. To do so, please run the following command:

```bash 
    make install
```

The make install command tests and tries to install all required developer dependencies.
if the automatic install fails, the script will show a link with further installation instructions.

After all dependencies have been installed, the following commands are ready to use.

```bash 
    make build          Builds the code base incrementally (fast) for dev.
    make bench          Runs all benchmarks across all crates.
    make check          Checks the code base for security vulnerabilities.
    make fix            Fixes linting issues as reported by clippy
    make format         Formats call code according to cargo fmt style
    make install        Tests and installs all make script dependencies
    make start          Start day with updating rust, pulling git, and build the project
    make run            Runs the binary defined in scripts/run.sh.
    make test           Runs all tests across all crates.
```

The scripts called by each make command are located in the [script folder.](build/scripts)

## Testing

Plese start a docker deamon before tesing.

The only way to run the entire test suite is via the `make test` command that calls the `test.sh` script. The reason is that the integraiton tests must be executed in strict order on a local machine. For integration tests on a CI service like BuildBuddy, the buildbuddy.yaml file holds the required configuration. When each task execute runs a docker demon, the all integration tets are executed in parallel.
