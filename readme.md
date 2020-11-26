# pawOS - microkernel written in Rust
Cat paws are cute. They deserve an OS.

Based on os.phil-opp.com guide to writing a kernel in Rust.
Kudos for them!

# How to run it
- Install qemu
- `rustup override set nightly`
    - `rustc --version` should show something like this: rustc 1.50.0-nightly (a0d664bae 2020-11-22)
- `rustup component add rust-src`
- `rustup component add llvm-tools-preview`
- `cargo install bootimage`
- `cargo run` should now start qemu with OS loaded

# How to test it
- TODO

## More info coming soon
Generally I'm pretty excited for this project. I'm still learning Rust and OS dev so the code may be rough around the edges but yeah... that's it for now. 