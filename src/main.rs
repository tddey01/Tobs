// 1 创建一个空String
// 2 通过字面值创建一个String
// 2 1、使用String::from()
// 2 2、使用str的方式
// 3 更新String
// 3 1、push_str
// 3 2、push
// 3 3、使用 "+" 合并字符串
// 3 4、 使用format
// 4 使用String 索引
// 5  str 索引
// 6 遍历
// 6 1、 chars
// 6 2、 bytes
fn main() {
    // 创建空的字符串
    let mut s0 = String::new(); // mut可以修改字符串
    s0.push_str("hello");
    println!("s0 = {}", s0);

    // 2 通过字面值创建一个String
    let s1 = String::from("init some thing");
    println!("{}", s1);

    let s1 = "init some thing".to_string();
    println!("{}", s1);


    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let ss = "  ！".to_string();
    s2.push_str(&ss); //引用方法
    println!("{}",s2);
    println!("ss = {}",ss);

    let mut s2 = String::from("tes");
    s2.push('m'); // 只能添加一个字符 需要使用单引号
    // s2.push('mx'); // error
    // s2.push("m");   // error
    println!("{}",s2);


    let s1 = "hello".to_string();
    let s2 =String::from(", world");
    let s3 = s1 + &s2;
    println!("s3 = {}",s3);
    // println!("s1 = {}",s1);  // error s1的所有权 全部赋值给了s3
    println!("s2 = {}",s2);

    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}",s341,s342,s343); //format！ 和 println! 类似 一样的
    println!("s344 = {}",s344);
    println!("s341 = {}",s341);
    println!("s342 = {}",s342);
    println!("s343 = {}",s343);

    let s4 = String::from("hello");
    // let s41 = s4[2]; // 不能被索引
    println!("s4 = {}",s4.len());

    let s4 = String::from("你好");
    println!("s4 = {}",s4.len());

    let hello = "你好";
    let h5 = &hello[0..3];
    println!("h5 = {}",h5);

    println!("hello world！");
}
