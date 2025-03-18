package main

/*
#cgo LDFLAGS: -lzkp -lm -ldl -L${SRCDIR}/lib/ -Wl,-rpath=${SRCDIR}/lib
#cgo gpu LDFLAGS: -lzkp -lm -ldl -lgmp -lstdc++ -lprocps -L/usr/local/cuda/lib64/ -lcudart -L${SRCDIR}/lib/ -Wl,-rpath=${SRCDIR}/lib
#include <stdlib.h>
#include "./lib/zkvm.h"
*/
import "C" //nolint:typecheck
import (
	// "encoding/json"
	"io/ioutil"
	"path/filepath"
	"fmt"
	"strings"

	// "github.com/scroll-tech/scroll/common/types"
)

func main() {
	// init
	C.init(C.CString(string("assets/")))
	
	// chunk test
	chunk := load_chunk_inputs("testdata/")
	chunk_input := "[" + strings.Join(chunk, ",") + "]"
	chunk_proof := C.generate_chunk_proof(C.CString(string(chunk_input)), C.CString("euclid"))
	defer C.free_proof(chunk_proof)
	
	// TODO: batch test

	// TODO: bundle test
}

func load_chunk_inputs(tdPath string) []string {
	blockStart := 10319966
	blockEnd := 10319974

	blocks := make([]string, blockEnd-blockStart)
	for block := blockStart; block < blockEnd; block++ {
		fileName := fmt.Sprintf("%d.json", block)
		filePath := filepath.Join(tdPath, fileName)
		blockWitness, err := ioutil.ReadFile(filePath)
		if err != nil {
			panic(err)
		}
		blocks[block-blockStart] = string(blockWitness)
	}
	return blocks
}
