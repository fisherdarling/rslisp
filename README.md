# SlytherLisp D7

This is the extra credit deliverable for Joseph and Fisher.

The deliverable is an experimental implementation of SlytherLisp in the Rust programming language. To run the
`main.rs` executable:

First install the latest nightly rust:

> `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`

And then run the code with:

> `cargo run`

The slytherlisp code that is executed is:

```
(define (add-new x y) 
    (+ x y)) 

(add-new 1 1)
```

The features are very minimal, but there is a handwritten, lookahead-based parser, and a couple
of builtin functions. The current environment supports `define`, `+`, and `*`.