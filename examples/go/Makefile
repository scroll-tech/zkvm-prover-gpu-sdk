SCROLL_ZKVM_VERSION=0.1.0-rc.8

download:
	./download.sh

prover:
	cp ../../interface/* lib/
	mkdir -p cache
	go build

run:
	RUST_LOG=info ./zkvm-prover-client
