# Make will use bash instead of sh
SHELL := /usr/bin/env bash

.PHONY: help
help:
	@echo ' Run Services:'
	@echo '    make run   		Runs the default binary.'
	@echo '    make cmdb   	Runs the cmdb service. Requires DBGW & SMDB'
	@echo '    make dbgw   	Runs the dbgw service. Requires SurrealDB'
	@echo '    make smdb   	Runs the smdb service. Requires DBGW'
	@echo '    make symdb   	Runs the symdb service. Requires DBGW'
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
	@echo '    make pull   		Pull all container images from the remote registry.'
	@echo '    make push   		Builds, tests, and pushes to git remote.'
	@echo '    make test   		Tests across all crates.'
	@echo '    make sbe   		Generates Rust bindings for SBE messages.'

# "---------------------------------------------------------"
# Run targets
# "---------------------------------------------------------"


.PHONY: run
run:
	@source scripts/run_default.sh


.PHONY: cmdb
cmdb:
	@source scripts/run_cmdb.sh


.PHONY: dbgw
dbgw:
	@source scripts/run_dbgw.sh


.PHONY: smdb
smdb:
	@source scripts/run_smdb.sh


.PHONY: symdb
symdb:
	@source scripts/run_symdb.sh


# "---------------------------------------------------------"
# Development make targets
# "---------------------------------------------------------"

.PHONY: build
build:
	@source scripts/build.sh


.PHONY: current
current:
	@source scripts/current.sh


.PHONY: rebuild
rebuild:
	@source scripts/rebuild.sh


.PHONY: remote
remote:
	@source scripts/remote.sh


.PHONY: check
check:
	@source scripts/check.sh


.PHONY: container
container:
	@source scripts/container.sh

.PHONY: doc
doc:
	@source scripts/doc.sh


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
	@source scripts/release.sh


.PHONY: push
push:
	@source scripts/push.sh


.PHONY: pull
pull:
	@source scripts/pull.sh


.PHONY: update
update:
	@source scripts/update.sh


.PHONY: test
test:
	@source scripts/test.sh


.PHONY: sbe
sbe:
	@source scripts/sbe.sh
