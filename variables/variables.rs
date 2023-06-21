// const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3; // 常量是绑定到一个常量名且不允许更改的值
/* 常量
1、常量不允许使用mut，不仅仅默认不可比那，而且自始至终不变，用constant代替let关键字声明，并且值的类型必须注明
2、常量在任意作用域内声明，包括全局作用域
3、常量只能设置为常量表达式，而不能是函数调用的结果或是只能在运行时计算得到的值
4、Rust常量的命名约定是全部字母都使用大写，并使用下划线分隔单词
 */
use std::io;
fn main() {
    // let mut x = 5; // 默认情况下变量是不可变的
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 遮蔽
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 12;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // 数据类型
    // Rust是一种静态类型的语言，它必须在编译期知道所有变量的类型
    // 标量类型：整型、浮点型、布尔型和字符
    /* 整型
    1、（i u）（8 16 32 64 128 size），i32默认
    2、整型数表达形式：98_222（十进制）、0xff（十六进制）、0o77（八进制）、0b1111_0000（二进制）、b'A'（字节仅u8）
    3、整数溢出：
    （1）debug模式下，Rust会检查整型溢出，若存在溢出则在编译时会panic
    （2）release模式下：Rust不检查会导致panic的整型溢出，相反当检测到整型溢出时，Rust会进行一种被称为二进制补码包裹的操作
                      大于该类型最大值的数值会被“包裹”成该类型能够支持的对应数字的最小值，例如，256->0，257->1
    （3）要显式处理溢出的可能性
        -- 使用wrapping_*方法在所有模式下进行包裹，例如wrapping_add
        -- 如果使用checked_*方法时发生溢出，则返回None值
        -- 使用overflowing_*方法返回该值和一个指示是否存在溢出的布尔值
        -- 使用saturating_*方法使值达到最大值或最小值
     */

    // 浮点数，默认64位
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32;

    // addition
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let difference = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Result in 0

    // remainder
    // let remainder = 43 % 5;

    // 布尔类型
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // 字符类型
    // let c = 'z';  // Rust的字符类型大小为4个字节
    // let z = 'Z';

    // 复合类型
    //（1）元组类型：将多种类型的多个值组合到一个复合类型中的一种基本方式
    //         元组的长度是固定的，声明后，它们的长度就无法增长或缩小
    // 没有任何值的元组()是一种特殊的类型，只有一个值，也写成()。该类型被称为单元类型，该值被称为单元值。如果表达式不返回任何其他值，就隐式地返回单元值
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; // 解构以便于将获取个别值
    // println!("The value of y is: {}", y);
    // println!("The value of x is: {}", tup.0); // 通过.的方式取出
    //（2）数组类型：与元组不同，数组地每个元素必须具有相同地类型，且数值具有固定长度
    // let a = [1, 2, 3, 4, 5]; // 数据分配在栈而不是堆
    // let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
    // let first = a[0];
    // let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}
