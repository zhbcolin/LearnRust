fn main() {
    // println!("Hello world!");

    // another_function(5);
    // print_labeled_measurement(5, 'h');

    // 语句：执行一些操作但不返回值的指令
    // let y = 6; 正确，语句
    // let x = (let y = 6); 错误，let语句并不返回值
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    let x = five(4);

    println!("The value of x is: {}", x);
}

// Rust代码中的函数和变量名使用下划线命名法（snake case）规范风格。在下划线命名法中，所有字母都是小写并使用下划线分隔单词。
// 源码中another_function定义在main函数之后，也可以定义在之前。Rust不关心函数定义于何处，只要定义了就行。
// fn another_function() {
//     println!("Another function.");
// }

// Rust可定义拥有参数，参数是特殊变量，是函数签名的一部分。
// 当函数拥有参数（形参）（parameter）时，可以为这些参数提供具体的值（实参）（argument）。
// 在函数签名中，必须声明每个参数的类型。
// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// 当一个函数有多个参数时，使用逗号分隔。
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// 带有返回值的函数
// 函数可以向调用它的代码返回值，并不对返回值命名，但要在箭头（->）后声明它的类型。
// 在Rust中，函数的返回值等同于函数体最后一个表达式的值。
// 使用return关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
fn five(x: i32) -> i32 {
    //5 // 表达式
    x + 1 // 表达式
    //x + 1; // 错误，语句没有返回值
}