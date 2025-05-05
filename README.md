### SYNOPSIS

Building a Scheme JIT from scratch and experimenting with runtime
feedback based optimizations.

Fix to learn.

### BUILD

Upgrade to Rust edition 2021, and you are on a `x86_64 Linux`
or `x86_64 OSX` machine. Then:

    cd src/scheme && cargo build --release

### RUN

    cd src/scheme && ./target/release/scheme scheme-src/fibo-unsafe.ss

### NOMENCLATURE

For Artoria Pendragon!
