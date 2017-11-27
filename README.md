# fst-wasm
This is my first try with WebAssembly through Rust.
```index.html``` returns the factorial of 5.

## Execution
Simply open the index.html file to see the application running.

### Retrace
In order to obtain the .wasm files I ran a few commands in the following order:

```shell
rustc +nightly --target wasm32-unknown-unknown -O factorial.rs --crate-type=cdylib
```

Which generates a WebAssembly (.wasm) file of the same name as the original Rust file.

```shell
wasm-gc factorial.rs small-factorial.wasm
```

```factorial.rs``` has been created in the prior step. Now wasm-gc helps us
minimize the code, for instance, by removing unused code. The resulting file
will be named as stated in the second argument, here ```small-factorial.wasm```.

## Needed tools
* [wasm-gc](https://github.com/alexcrichton/wasm-gc)

## References
* [Native Wasm Target - News](https://gist.github.com/steveklabnik/d86491646b9e3e420f8806a286ec8e92)
* [Minimal Rust & WebAssembly example](https://www.hellorust.com/demos/add/index.html)
* [Code](https://gist.github.com/steveklabnik/d86491646b9e3e420f8806a286ec8e92) (associated with the secondly enumerated item).
