fn main() {
    // let number = 3;
    //
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    //
    // // if表达式，使用else if处理多重条件
    // // Rust并不会尝试自动地将非布尔值转换为布尔值
    // // 指定满足条件的第一个语句
    //
    // // 在let语句中使用if
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // // let number = if condition { 5 } else { "six" }; // 错误，每个分支的可能返回值都必须是相同类型
    // println!("The value of number is: {}", number);

    // 使用循环重复执行
    // 使用loop重复执行代码
    // loop {
    //     println!("again!");
    // }
    // break和continue
    // 如果存在嵌套循环，break和continue应用于此时最内层的循环。
    // 可以选择在一个循环上指定一个循环标签，然后将标签与break或continue一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    // println!("End count = {}", count);

    // 从循环返回
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}