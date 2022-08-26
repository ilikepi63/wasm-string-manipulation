# Wasmtime String Manipulation example. 

## Introduction

NOTE: THIS IS NOT A PRODUCTION EXAMPLE. There are far better and more safer ways to do this, such as using wit-bindgen (https://github.com/bytecodealliance/wit-bindgen). This is only to understanding how lower-level interactions with wasm memory can work.

This is just a quick example of how someone *could* manipulate memory inside of a wasmtime instance. 

## Explanation

### wasm-string-manipulation-lib

This is just a library that exports a certain amount of wasm functions for manipulating its internal memory. Note: there is a lack of a "free" function for freeing an allocated memory block.

### wasm-string-manipulation-bin

This is a runner that will point to a compiled wasm module, allocate memory inside of it and call functions to operate on it. 

