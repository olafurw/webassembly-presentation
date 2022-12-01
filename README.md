# Rust Slides

This is a very bespoke project I made to do a presentation about WebAssembly, so the presentation itself is also in WebAssembly (using Rust/Yew)

Browse at will and be inspired if you want but note the code isn't made for public use.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install trunk

git clone git@github.com:olafurw/webassembly-presentation.git
cargo build

trunk build
trunk serve
```
