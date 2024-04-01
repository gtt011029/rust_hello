/*
    rust 无需垃圾回收
    所有权是rust最为与众不同的特性
    所有程序都必须管理自己运行时使用计算机内存的方式
    （通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。
    如果违反了任何规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序）

 */
/* 
 入栈比在堆上分配内存要快
    入栈，直接放在顶部
    存在堆中， 要先找到合适的内存空间，然后进行分配

    访问堆上的数据比访问栈上的数据慢 */


    /*
    所有权系统的作用：（管理堆数据）
    1、跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量
    2、清理堆上不再使用的数据，确保不会耗尽空间 


    a、Rust中的每一个值都有一个所有者（owner）
    b、值在任一时刻有且只有一个所有者
    c、当所有者（变量）离开作用域，这个值将被丢弃。
     */


    pub fn str_ship(){
        // String 可以管理并分配到堆上的数据，所以能够存储在编译时未知大小的文本
        

/*      对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：

        必须在运行时向内存分配器（memory allocator）请求内存。
        需要一个当我们处理完 String 时将内存返回给分配器的方法。 */


        /* 
        为了确保内存安全，
        在 let s2 = s1; 之后，Rust 认为 s1 不再有效，
        因此 Rust 不需要在 s1 离开作用域后清理任何东西 */

        let mut s1 = String::from("hello");
        let mut s2 = s1;  // 这个操作被认为移动（move），这个时候s1就无效了
        
        // s1.push_str(", world"); // 所以这个地方就会报错， 因为s1被移动了，无效了
        // println!("s1 is {s1}");


        let s3 = s2.clone(); //这样是可以的
        s2.push_str(", world");
        println!("s2 is {s2}");
        println!("s3 is {s3}");

    }
 

 pub fn fun_ship() {
    let s = String::from("hello");
    takes_ownership(s);

    // 会报错 value borrowed here after move， 这个时候就不能用s这个变量了， （ps： 简直无语）
    // println!("current s is {s}"); 

    let x = 5;
    makes_copy(5)

 }

 fn takes_ownership(some_string: String) {
    println!("{some_string}")
 }

 fn makes_copy(some_interger: i32) {
    println!("{some_interger}")
 }

 fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
 }

 fn takes_and_give_back(a_string: String) -> String {
    a_string
 }

 pub fn fun_ship_2() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);
    // println!("这个地方还可以使用S2吗 {s2}") // 结论： 不行
 }

 // 引用与借用
 /*
 引用像一个指针，因为 它是一个地址
  */
 pub fn fun_ship_3() {
    let mut s1 = String::from("hello");
    // let len = calculate_length(&s1);  // 这边传过去的是s1的指针
    // println!("the length of {s1} is {len}"); // 可以看出这边还是可以继续使用s1，s1依旧有效， 


    let r1 = &mut s1;
    // 报错 a local variable with a similar name exists: `s1`
    // 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。这些尝试创建两个 s 的可变引用的代码会失败：
    // let r2 = &mut s2; 


    // 可变引用
    // change(&mut s1);
    // change(&mut s1); // 都可以
    // println!("看一下s1有没有变：{s1}"); // 变了

    // println!("result: {}", dangling())
 }

 // 我们将创建一个引用的行为称之为”借用“
 fn calculate_length(s: &String) -> usize {
    // s.push_str("修改字符串"); // 这边报错（但是确实有时候有这样的需求）（默认不允许修改引用的值）
    s.len()
 }

 fn change(some_string: &mut String) {
    some_string.push_str(", world")
 }

 /*
 悬垂引用：
    是其指向的内存可能已经被分配给其他持有者 
    Rust中的编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
  */

/*   fn dangling() -> &String { // 这边会报错
    let s: String = String::from("hello"); // 这玩意在这个作用域外就会被清除
    &s
  } */

    fn dangling2() -> String { // 这样是可以的， 这边变量的所有者也会变更
    let s: String = String::from("hello");
    s
  }


  /* 
  引用的规则：
  在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
  引用必须总是有效的

   */



  /*
  slice: 可以和go 的 slice做比较 
  slice 允许你引用集合中一段连续的元素序列
  slice是一种引用， 没有所有权（ps： 是否可变呢）
   */

pub fn fun_ship_4() {
    let mut s = String::from("hello world");
    // let word = first_word(&s);
    // println!("字符串长度为： {}", word);
    // println!("空格对应的字节： {}", b' ') // 32


    let word = slice_test(&s);
    // s.clear(); // 报错， 这边是可变引用
    println!("the first word is: {}", word); // 这边使用了word， 就默认不可变引用仍然有效， 所以clear会报错
}

fn first_word(s: &String) -> usize {
    // as_bytes: 将String转化为字节数组
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // item: 集合中元素的引用
        // i 为对应的下标
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 入参为一个不可变引用
fn slice_test(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
