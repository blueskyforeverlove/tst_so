1.编译动态链接库
cargo build --lib --release
2.编译Rust可执行程序main
RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --bin main --release
3.编译C可执行程序cmain
cd src/bin/ && gcc cmain.c -ltst_so -L ../../target/release -o ../../target/release/cmain -Wl,-rpath,./
4.执行Rust程序
cargo run --release
或者
cd target/release && ./main

需要注意的是直接cargo run会重新生成main,这个main没有带RUSTFLAGS
5.执行C程序
cd target/release && ./cmain