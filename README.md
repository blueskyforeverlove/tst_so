1.**编译动态链接库** <br/>
&emsp;cargo build --lib --release <br/>
2.**编译Rust可执行程序main** <br/>
&emsp;RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --bin main --release <br/>
3.**编译C可执行程序cmain** <br/>
&emsp;cd src/bin/ && gcc cmain.c -ltst_so -L ../../target/release -o ../../target/release/cmain -Wl,-rpath,./ <br/>
4.**执行Rust程序** <br/>
&emsp;cargo run --release <br/>
&emsp;或者 <br/>
&emsp;cd target/release && ./main <br/>
<br/>
&emsp;需要注意的是直接cargo run会重新生成main,这个main没有带RUSTFLAGS <br/>
5.**执行C程序** <br/>
&emsp;cd target/release && ./cmain <br/>
