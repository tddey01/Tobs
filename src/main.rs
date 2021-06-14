
//  1 类似于c语言的方法定义
enum IpAddKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

//  2 rust语言提倡的方法定义
enum Ipaddr2 {
    V4(String),
    V6(String),
}

//  3 可以是不同类型
enum Ipaddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//  4 经典用法
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}
// 等同于
// struct QuiMessage：// 类单元结构体
// struct MoveMessage{
//  x: i32,
//  y: i32,
// }
// struct WriteMessage(String){
// struct Change(i32,i32,i32)

//  5 枚举类型的方法以及 match
impl Message {
    fn prin(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x={},y={}", x, y),
            Message::Change(a, b, c) => println!("Change a={}, b={},c={}", a, b, c),
            _ => println!("Write")
            // Message::Write(&s) => println!("Write={}", &s),
        }
    }
}

fn main() {
    let il = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let i2 = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };


    let i1 = Ipaddr2::V4(String::from("127.0.0.1"));
    let i2 = Ipaddr2::V6(String::from("::1"));


    let i1 = Ipaddr3::V4(127, 0, 0, 1);
    let i2 = Ipaddr3::V6(String::from("::1"));


    let quit = Message::Quit;
    quit.prin();
    
    let mo = Message::Move{x:10,y:20};
    mo.prin();
     
    let wri = Message::Write(String::from("hello"));
    wri.prin();

    let change = Message::Change(1,2,3);
    change.prin();



    println!("hello world！")
}
