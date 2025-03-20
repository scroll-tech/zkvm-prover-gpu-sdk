export SCROLL_ZKVM_VERSION=0.1.0-rc.10

download:
	./download.sh

prover:
	cp ../../interface/* lib/
	mkdir -p assets/cache
	go build

run:
	RUST_LOG=info ./zkvm-prover-client
