
cd prover/
cargo build --release
# TODO: add version control for libzkp.so
cp ./target/release/libzkp.so ../interface/
