package main

import (
	"fmt"
  "log"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

func main() {
	bytes, err := wasm.ReadBytes("my_lib.wasm")
  if err != nil {
    log.Fatalf("error reading wasm: %s", err)
  }
	instance, err := wasm.NewInstance(bytes)
  if err != nil {
    log.Fatalf("error instantiating wasm: %s", err)
  }
	defer instance.Close()

	sum := instance.Exports["sum_of_squares"]
	result, err := sum(100)
  if err != nil {
    log.Fatalf("error calling wasm library: %s", err)
  }

	fmt.Println(result)
}

