curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install trunk

git clone git@github.com:olafurw/webassembly-presentation.git
cargo build

trunk build
trunk serve
