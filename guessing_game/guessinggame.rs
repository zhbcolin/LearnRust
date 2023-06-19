use std::io;
// 默认情况下，Rust会将少量标准库中定义的程序项（item）引入到每个程序的作用域中，这些项称作prelude
// 如果需要的类型不在prelude中，必须使用use语句显式地将其引入作用域
// std::io库提供很多有用的功能，包括接收用户输入的功能
use std::cmp::Ordering;
// 也是一个枚举，成员是Less、Greater和Equal
use rand::Rng;
// Rng是一个trait，定义了随机数生成器实现的方法

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // rand::thread_rng函数来提供将要使用的特定随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取seed
    // gen_range(start..end); use rand::Rng语句引入的Rng trait定义

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // let声明来创建变量，在Rust中，变量默认是不可变的，使用mut来使一个变量可变
        // = 将某个值绑定在变量上，String::new返回一个String的新实例，UTF-8
        // ::new中::语法表明mew是String类型的一个关联函数

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // std::io::stdin函数返回一个std::io::Stdin的实例，代表终端标准输入句柄的类型
        // .read_line(&mut guess)调用read_line方法从标准输入句柄获取用户输入
        // read_line的全部工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中（不覆盖其内容），需要可变字符串参数，以便改方法可以更改字符串的内容
        // &引用，允许多处代码访问同一处数据，而无需在内存中多次拷贝，引用默认是不可变的
        // 使用Result类型来处理潜在的错误 .expect("Failed to read line");
        // read_line将用户输入放置到传递给它的字符串中，并返回一个值to::Result（Result的一个特化版本）
        // Result类型是枚举，持有固定集合的值，称为枚举的成员
        // 枚举往往与条件表达式match一起使用，可以方便地根据枚举值是哪个成员来执行不同的代码
        // Result的成员是Ok和Err，io::Result的实例拥有expect方法

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Rust允许用一个新值来遮蔽（shadow）guess之前的值，这允许复用
        // 将这个新变量绑定到guess().trim().parse()，String实例的trim方法会去除字符串开头和结尾的空白字符
        // 如果parse能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的Ok，与match第一个分支的模式相匹配，返回数字num
        // 如果parse不能将字符串转换为一个数字，它会返回一个包含更多错误信息的Err，会匹配第二个分支Err(_)模式，_是一个通配符值，用来匹配所有Err值

        println!("You guessed: {}", guess);
        // {}是预留在特定位置的占位符

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };
        // 一个match表达式由分支（arm）构成。一个分支包含一个用于匹配的模式（pattern），给到match的值与分支模式相匹配时，应该执行对应分支的代码
        // let guess = String::new()时，guess是String类型
        // secret_number是数字类型，Rust默认使用i32
    }
}