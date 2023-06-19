fn main() {
    println!("Hello, world!");
}


/*
1、fn main() {}
（1）main函数是每一个可执行Rust程序中运行的第一个代码
（2）不带参数也没有返回值，如果有参数，那么会放到括号()内
（3）函数主体用大括号{}括起来，一种好的代码风格是将左大括号放在函数声明的同一行，且之间带有一个空格
2、println!("Hello, world!");
（1）Rust风格的缩进使用4个空格，而不是制表符
（2）println!调用Rust宏，如果改为调用函数，则应该将其输入为println，当看到一个!，则意味着调用的是宏而不是普通的函数
3、编译和运行
（1）rustc:编译器编译程序 rustc xxx.rs
（2）.rs源文件 .exe可执行文件 .pdb调试信息文件
4、Cargo
（1）可以用cargo build构建项目
（2）可以用cargo run一步构建并运行项目
（3）可以用cargo check构建项目而无需生成二进制文件来检查错误
 */