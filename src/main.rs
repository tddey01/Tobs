// 1 Option 是标准库定义的一个枚举，形式
// enum Option<T>  泛型
// {
//  Some(T), 表示一个值
//  None
// }

// 2 使用方式
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let mut temp = 0; // 可变的值
    match y {
        Some(i) => { temp = i; }
        None => { println!("do nothing"); }
    };
    let sum = x + temp;
    println!("sum = {}", sum);
    let result = plus_one(y);
    match result {
        Some(i) => println!("result = {}", i),
        None => println!("No None")
    }

    println!("hello world！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}