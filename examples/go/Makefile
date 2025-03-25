export SCROLL_ZKVM_VERSION=0.1.1-rc.3

download:
	./download.sh

prover:
	cp ../../interface/* lib/
	mkdir -p assets/cache
	go build

run:
	RUST_LOG=info ./zkvm-prover-client
