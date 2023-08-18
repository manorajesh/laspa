# läspa
![Build Status](https://github.com/manorajesh/laspa/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/manorajesh/laspa/branch/master/graph/badge.svg?token=2CN1LLRK4P)](https://codecov.io/gh/manorajesh/laspa)
![Downloads](https://img.shields.io/crates/d/laspa)
![Version](https://img.shields.io/crates/v/laspa)
![License](https://img.shields.io/crates/l/laspa)

A toy language I designed to be as easy as possible to implement. 
Reminiscent of [lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language)), 
läspa uses [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) 
for basic arithmetic and for function calls. With a basic interpreter implemented, I plan to
implement a compiler with the help of [LLVM](https://llvm.org/docs/LangRef.html#type-system) and plain machine-code generation.

## Installation
```shell
cargo install laspa
```

You will need the llvm toolchain to build the executable. Clang is also used for linking.
```shell
brew install llvm && export LLVM_SYS_160_PREFIX='/usr/local/opt/llvm@16'
```

## Usage
See [this test file](https://github.com/manorajesh/laspa/blob/master/examples/test.laspa) for example syntax.
```
A simple Lisp-like language built with Rust

Usage: laspa [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to build

Options:
  -O, --optimization-level <OPTIMIZATION_LEVEL>  Optimization level for the compiler [default: 1]
  -i, --interpret                                Interpret the file
  -v, --verbose...                               Verbose output
  -o, --executable-name <EXECUTABLE_NAME>        Executable name [default: main]
      --jit                                      Execute IR with JIT
  -h, --help                                     Print help (see more with '--help')
  -V, --version                                  Print version
  ```

## Why
I was reading an [article](https://mhdm.dev/posts/sb_lower_bound/) on the fastest implementation of a binary search algorithm.
I saw `llvm` and thought to myself: "Hmm, wouldn't it be interesting to make a language." The rest is history.


#### Important Code
The `lex`, `parse`, and `eval` functions are the meat of the execution of the language. Those familiar with language
development will recognize those names. `llvm.rs` is crucial to generating and compiling the IR for LLVM executations.
