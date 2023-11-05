# Quant-Engine

![Audit][audit-url]
![Clippy][clippy-url]
![Tests][test-url]
[![codecov][codecov-badge]][codecov-url]

[audit-url]: https://github.com/marvin-hansen/quant-engine/actions/workflows/audit.yml/badge.svg

[clippy-url]: https://github.com/marvin-hansen/quant-engine/actions/workflows/rust-clippy.yml/badge.svg

[codecov-badge]: https://codecov.io/gh/marvin-hansen/quant-engine/graph/badge.svg?token=21Z5EN77FP

[codecov-url]: https://codecov.io/gh/marvin-hansen/quant-engine

[test-url]: https://github.com/marvin-hansen/quant-engine/actions/workflows/run_tests.yml/badge.svg

## Memgraph

### Create new memgraaph container

docker run -it -p 7687:7687 -p 7444:7444 -p 3000:3000 --name memgraph memgraph/memgraph-platform

### Start / stop memgraaph

docker start memgraph

docker stop memgraph

### web console

http://localhost:3000/

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

The scripts called by each make command are located in the [script folder.](scripts)