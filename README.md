
# SCROLL ZKVM GPU PROVER

## How to use the sdk

In the directory `interface/` we have two files:
1. dynamic library `libzkp.so`;
2. C header file `zkvm.h`.

You can include the header file in your program and link your binary with the dynamic library.

### Example in Golang

You can build the example tester in golang using cmd `cd examples/go/ && make prover`.
