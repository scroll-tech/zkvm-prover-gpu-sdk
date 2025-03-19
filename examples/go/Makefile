SCROLL_ZKVM_VERSION=0.1.0-rc.8

prover:
	# if prover.so is already there, then skip the build
	./download.sh
	cp ../../interface/* lib/
	go build
	mkdir -p cache

run: prover
	RUST_LOG=info ./zkvm-prover-client
