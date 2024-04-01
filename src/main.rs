use std::io; // 从标准库中引入输入输出功能

// mod test; // 导入test文件
mod ownership;

// 在 Windows 和 Linux 上，使用 Ctrl+Shift+A 快捷键。


fn guess() {
    println!("guess the number");
    println!("Please input your guess.");
    // String::new() 会生成一个string的实例
    let mut guess = String::new();
    // &mut guess  让用户输入存储到 guess字符串中
    // & 为引用，允许多处代码访问同意出数据， 无需在内存中多次拷贝，rust的一个主要优势就是安全而简单的操纵引用
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // {} 预留在特定位置的占位符
    println!("you guessed :{guess}");

    // let apples = 5; // 不可变的变量
    // let mut bananas = 5; //可变的变量

    // cargo build 构建项目
    // cargo run 一步构建并运行项目
    // cargo check 在不生成二进制文件的情况下构建项目来检查错误
    // 有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 target/debug 目录。
    // println!("Hello, world!， 怎么没有build' run也可以");
}

fn variable() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6; // 报错 cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");
}

fn hidden() {
    let x = 5;
    println!("1: the value of is: {x}");
    let x = x + 1;
    println!("2: the value of is: {x}");
    {
        // 开辟了一个独立的作用域
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }
    println!("the value of is: {x}")
}

fn arr() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let ele = a[index];

    println!("The value of the element at index {index} is: {ele}");
}

fn anther_fun(x: i32) {
    println!("the param is {x}")
}

// 返回值的函数 加 -> 返回值类型
// 注意表达式和语句的区别(其实就是没有return的简易写法)
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_test() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 这算是三元表达式吗， 不太算吧
    println!("the number is {number}")
}

fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("current counter is {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}")
}


// 循环标签（loop_label）: 用于在内层循环中break外层循环  首字符 '
fn loop_loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");
}

fn while_test() {
    let mut number = 3;
    while number != 0 {
        println!("{number} !");
        number -= 1;
    }
    println!("LIFTOFF!!!")
}

fn for_test() {
    // let a = [11, 22, 33, 44, 55];
    // let mut index = 0;

    // for ele in a {
    //     println!("the value is {ele}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!")
}

fn main() {
    // test::test(); // 调用test方法
    // variable()
    // hidden()
    // arr()
    // anther_fun(212)
    // let ret = five();
    // println!("return {ret}")
    // let ret2 = plus_one(5);
    // println!("the return value is {ret2}")

    // if_test()

    // loop_test()
    // loop_loop_test()

    // while_test()
    // for_test()

    // ownership::str_ship()
    // ownership::fun_ship()
    // ownership::fun_ship_2()
    // ownership::fun_ship_3()
    ownership::fun_ship_4()

}
