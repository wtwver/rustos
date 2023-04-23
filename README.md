# rustos

cargo new blog_os --bin --edition 2018
cargo build

//cross compile
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf 
