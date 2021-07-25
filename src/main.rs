use std::vec;

// 1 创建空的 vector: Vec<T>
// 2 创建包含初始值的vector
// 3 丢弃vector
// 4 读取元素
// 5 更新
// 6 遍历
// 7 使用枚举

fn main() {
    // 1 创建空的的vector
    // let mut  v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    //  v.push(1);

    // 2 创建包含初始值的vector
    let v = vec![1, 2, 3];

    //  3 丢弃
    {
        //作用域
        let v1 = vec![1, 2, 3];
    }

    // 4 读取元素

    let one: &i32 = &v[0];
    // let four : &i32 = &v[3];
    println!("one = {}", one);
    println!("one = {}", *one);

    // 推荐使用第二种方法
    match v.get(1) {
        Some(value) => println!("two = {}", value),
        _ => println!("None do nting"),
    }

    //  更新操作
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    // match v2.get(2) {
    //     Some(value) => println!("v2 = {}", value),
    //     _ => println!("None do nting"),
    // }
    //6  遍历
    //  （1） 不可变遍历
    for i in &v2 {
        println!("i  = {}", i);
    }
    // （2） 可变的遍历

    for i in &mut v2 {
        *i += 1;
        println!("i += {}", i);
    }

    //  7
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001),
    ];

    // 8 补充

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("first = {}", first); //不可变的值

    println!("hello world！");
}
