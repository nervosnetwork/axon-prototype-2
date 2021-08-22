.PHONY: force

force:

build: force
	@echo 'parse env "gcc_typehash" thru capsule build --env gcc_typehash=test_gcc, supported list pls refer common/build.rs'
	@echo 'use default value test_gcc for gcc_typehash'
	cargo fmt
	capsule clean
	capsule build

build-dev:
	cargo fmt
	capsule build --env gcc_typehash=dev_gcc

build-lina:
	cargo fmt
	capsule build --env gcc_typehash=lina_gcc

build-aggron:
	cargo fmt
	capsule build --env gcc_typehash=aggron_gcc

clean:
	cargo clean
	capsule clean

fmt:
	cargo fmt

prepare_env:
	cargo install ckb-capsule --force --git https://github.com/LycrusHamster/capsule.git --rev 40fdb5e2
	cargo install moleculec --version 0.7.0

generate_mol_2_rs:
	moleculec --language rust --schema-file ./common-raw/src/molecule/common.mol > ./common-raw/src/molecule/common.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/checker_bond.mol > ./common-raw/src/molecule/cell/checker_bond.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/checker_info.mol > ./common-raw/src/molecule/cell/checker_info.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/code.mol > ./common-raw/src/molecule/cell/code.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sidechain_bond.mol > ./common-raw/src/molecule/cell/sidechain_bond.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sidechain_config.mol > ./common-raw/src/molecule/cell/sidechain_config.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sidechain_fee.mol > ./common-raw/src/molecule/cell/sidechain_fee.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sidechain_registry.mol > ./common-raw/src/molecule/cell/sidechain_registry.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sidechain_state.mol > ./common-raw/src/molecule/cell/sidechain_state.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/sudt_token.mol > ./common-raw/src/molecule/cell/sudt_token.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/cell/task.mol > ./common-raw/src/molecule/cell/task.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/witness/anyone_shutdown_sidechain.mol > ./common-raw/src/molecule/witness/anyone_shutdown_sidechain.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/witness/collator_submit_tasks.mol > ./common-raw/src/molecule/witness/collator_submit_tasks.rs
	moleculec --language rust --schema-file ./common-raw/src/molecule/witness/collator_shutdown_sidechain.mol > ./common-raw/src/molecule/witness/collator_shutdown_sidechain.rs
	cargo fmt
	@echo 'check if any witness left'
