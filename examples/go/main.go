package main

/*
#cgo LDFLAGS: -lzkp -lm -ldl -L${SRCDIR}/lib/ -Wl,-rpath=${SRCDIR}/lib
#cgo gpu LDFLAGS: -lzkp -lm -ldl -lgmp -lstdc++ -lprocps -L/usr/local/cuda/lib64/ -lcudart -L${SRCDIR}/lib/ -Wl,-rpath=${SRCDIR}/lib
#include <stdlib.h>
#include "./lib/zkvm.h"
*/
import "C" //nolint:typecheck
import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	var hardfork_name = "euclidv2"
	// init
	C.init(C.CString(string("assets/")))
	
	// get vks
	chunk_vk := C.get_chunk_vk(C.CString(string(hardfork_name)))
	batch_vk := C.get_batch_vk(C.CString(string(hardfork_name)))
	bundle_vk := C.get_bundle_vk(C.CString(string(hardfork_name)))
	defer C.free_vk(chunk_vk)
	defer C.free_vk(batch_vk)
	defer C.free_vk(bundle_vk)

	go_chunk_vk := C.GoString(chunk_vk)
	go_batch_vk := C.GoString(batch_vk)
	go_bundle_vk := C.GoString(bundle_vk)
	fmt.Println("Chunk VK:", go_chunk_vk)
	fmt.Println("Batch VK:", go_batch_vk)
	fmt.Println("Bundle VK:", go_bundle_vk)

	// chunk test
	chunk_input := loadChunkInputs("testdata/")
	chunk_proof := C.generate_chunk_proof(C.CString(string(chunk_input)), C.CString(string(hardfork_name)))
	defer C.free_proof(chunk_proof)
	go_chunk_proof := C.GoString(chunk_proof)
	fmt.Println("Chunk proof:", go_chunk_proof)
	fmt.Println("Succeed to generate chunk proof!")
	
	// TODO: add verifier for chunk proof

	// TODO: batch test

	// TODO: bundle test
}

func loadChunkInputs(tdPath string) string {
	blockStart := 10319966
	blockEnd := 10319974

	blocks := make([]string, blockEnd-blockStart)
	for block := blockStart; block < blockEnd; block++ {
		fileName := fmt.Sprintf("%d.json", block)
		filePath := filepath.Join(tdPath, fileName)
		blockWitness, err := os.ReadFile(filePath)
		if err != nil {
			panic(err)
		}
		blocks[block-blockStart] = string(blockWitness)
	}
	chunkInputs := "[" + strings.Join(blocks, ",") + "]"
	return chunkInputs
}
