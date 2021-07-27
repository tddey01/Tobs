// HashMap < K, v>
// 创建HashMap
// 读取
// 遍历
// 更新
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("Blue");
    if let Some(v) = scores.get(&k) {
        // v = 10
        println!("v = {}", &v);
    }

    let k = String::from("Yellow");
    let v = scores.get(&k);
    match v {
        Some(value) => println!("v = {}", value),
        None => println!("None doting"),
    }

    println!("++++++++++++++++++++++++++++++++");
    // 遍历HashMap  会以任意的顺序遍历出来
    for (key, value) in &scores {
        println!("key = {}, value = {}", key, value)
    }
    println!("++++++++++++++++++++++++++++++++");

    // 更新HashMap
    let mut ss: HashMap<String, i32> = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 3);
    println!("ss = {:?}", &ss);

    // 键不存在的时候才插入
    let mut ss1: HashMap<String, i32> = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss = {:?}", &ss1);

    // 根据旧的值来更新一个值
    let text = "hello  world wornderful world";
    let mut map = HashMap::new();
    for world in text.split_ascii_whitespace() {
        let count = map.entry(world).or_insert(0);
         *count += 1;
    }
        println!("map = {:?}", map);
    println!("hello world！");
}
