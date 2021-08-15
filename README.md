1.编译动态链接库
cargo build --lib --release
2.编译可执行程序
RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --bin main --release
3.执行
cargo run --release
或者
cd target/release && ./main

需要注意的是直接cargo run会重新生成main,这个main没有带RUSTFLAGS