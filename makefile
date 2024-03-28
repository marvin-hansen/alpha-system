# Make will use bash instead of sh
SHELL := /usr/bin/env bash

.PHONY: help
help:
	@echo ' Run Services:'
	@echo '    make run   		Runs the default binary.'
	@echo '    make cmdb   	Runs the cmdb service. Requires DBGW & SMDB'
	@echo '    make dbgw   	Runs the dbgw service. Requires SurrealDB'
	@echo '    make qdgw   	Runs the qdgw service. Requires DBGW & SMDB'
	@echo '    make smdb   	Runs the smdb service. Requires DBGW'
	@echo '    make symdb   	Runs the symdb service. Requires DBGW'
	@echo '    make vex   		Runs the vex service. '
	@echo ''
	@echo ' Development:'
	@echo '    make build   	Builds the code base incrementally (fast) for dev.'
	@echo '    make current   	Builds the current target incrementally (fast) for dev.'
	@echo '    make rebuild   	Syncs dependencies and builds the code base from scratch (slow).'
	@echo '    make check   	Checks the code base for security vulnerabilities.'
	@echo '    make container   Builds the container images.'
	@echo '    make doc   		Builds documentation for the project.'
	@echo '    make fix   		Fixes linting issues as reported by clippy.'
	@echo '    make format   	Formats call code according to cargo fmt style.'
	@echo '    make install   	Tests and installs all make script dependencies.'
	@echo '    make update   	Update rust, pulls from git remote.'
	@echo '    make test   		Tests across all crates.'
	@echo '    make sbe   		Generates Rust bindings for SBE messages.'

# "---------------------------------------------------------"
# Service make targets
# "---------------------------------------------------------"
.PHONY: run
run:
	@source scripts/run.sh


.PHONY: cmdb
cmdb:
	@source scripts/service_cmdb.sh


.PHONY: dbgw
dbgw:
	@source scripts/service_dbgw.sh


.PHONY: qdgw
qdgw:
	@source scripts/service_qdgw.sh


.PHONY: smdb
smdb:
	@source scripts/service_smdb.sh


.PHONY: symdb
symdb:
	@source scripts/service_symdb.sh


.PHONY: vex
vex:
	@source scripts/service_vex.sh

# "---------------------------------------------------------"
# Development make targets
# "---------------------------------------------------------"

.PHONY: build
build:
	@source scripts/bazel/build.sh


.PHONY: current
current:
	@source scripts/bazel/current.sh


.PHONY: rebuild
rebuild:
	@source scripts/bazel/rebuild.sh


.PHONY: check
check:
	@source scripts/check.sh


.PHONY: container
container:
	@source scripts/bazel/image.sh


.PHONY: doc
doc:
	@source scripts/bazel/doc.sh


.PHONY: fix
fix:
	@source scripts/fix.sh


.PHONY: format
format:
	@source scripts/format.sh


.PHONY: install
install:
	@source scripts/install_deps.sh


.PHONY: release
release:
	@source scripts/bazel/release.sh


.PHONY: update
update:
	@source scripts/update.sh


.PHONY: test
test:
	@source scripts/bazel/test.sh


.PHONY: sbe
sbe:
	@source scripts/sbe.sh
