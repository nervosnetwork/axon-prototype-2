
build:
	echo 'parse env gcc_typehash thru capsule build --env gcc_typehash=test_gcc, supported list pls refer common/build.rs'
	fmt
	capsule build

clean:
	cargo clean

fmt:
	cargo fmt

prepare_env:
	cargo install ckb-capsule --force --git https://github.com/LycrusHamster/capsule.git --rev 40fdb5e2
