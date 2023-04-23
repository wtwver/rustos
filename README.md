# rustos

cargo new blog_os --bin --edition 2018

cargo build

//cross compile

rustup target add thumbv7em-none-eabihf

cargo build --target thumbv7em-none-eabihf 

rustup override set nightly

rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage