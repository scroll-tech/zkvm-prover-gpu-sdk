
# SCROLL ZKVM GPU PROVER

## Prerequisite

The minimum required NVIDIA driver is `>=525.60.13`. You can refer to [nvidia drivers](https://www.nvidia.com/en-us/drivers/) to find the driver version for your GPU.

## How to use the sdk

In the directory `interface/` we have two files:
1. dynamic library `libzkp.so`;
2. C header file `zkvm.h`.

You can include the header file in your program and link your binary with the dynamic library.

### Example in Golang

You can build the example tester in golang using cmd `cd examples/go/ && make prover`.

## TODO

add version control for dynamic libraries.
