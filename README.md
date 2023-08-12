# läspa
![Build Status](https://github.com/manorajesh/laspa/actions/workflows/MacOS.yml/badge.svg)
![Build Status](https://github.com/manorajesh/laspa/actions/workflows/Linux.yml/badge.svg)
![Build Status](https://github.com/manorajesh/laspa/actions/workflows/Windows.yml/badge.svg)

A toy language I designed to be as easy as possible to implement. 
Reminiscent of [lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language)), 
läspa uses [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) 
for basic arithmetic and for function calls. With a basic interpreter implemented, I plan to
implement a compiler with the help of [LLVM](https://llvm.org/docs/LangRef.html#type-system) and plain machine-code generation.

## Installation
```shell
git clone https://github.com/manorajesh/laspa.git && cd laspa
cargo test
```

The language is currently only a library with a full test suite. 

## Usage
See [this test file](https://github.com/manorajesh/laspa/blob/master/examples/test.laspa) for example syntax.

## Why
I was reading an [article](https://mhdm.dev/posts/sb_lower_bound/) on the fastest implementation of a binary search algorithm.
I saw `llvm` and thought to myself: "Hmm, wouldn't it be interesting to make a language." The rest is history.


#### Important Code
The `lex`, `parse`, and `eval` functions are the meat of the execution of the language. Those familiar with language
development will recognize those names.
