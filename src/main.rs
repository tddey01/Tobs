//1、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复.
//2、在函数定义中使用泛型。
//3、在结构体中使用泛型。
//4、枚举中的泛型。
//5、方法中的泛型。
//6、总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
////--------没有泛型-----------
// for i32
// fn laragest_i32(list: &[i32]) -> i32 {
//     let mut laragest = list[0];
//     for &item in list.iter() {
//         if item > laragest {
//             laragest = item;
//         }
//     }
//     laragest
// }

// fn laragest_char(list: &[char]) -> char {
//     let mut laragest = list[0];
//     for &item in list.iter() {
//         if item > laragest {
//             laragest = item
//         }
//     }
//     laragest
// }

// fn main() {
//     // let number_list = vec![1,2,23,34,8,100];
//     // let max_number = laragest_i32(&number_list);
//     // println!("max_number = {}",max_number);

//     // let char_list = vec!['a','y','b'];
//     // let max_char =  laragest_char(&char_list);
//     // println!("max_chat = {}",&max_char);

//     let number_list = vec![1, 2, 3, 5, 7, 88, 92];
//     let max_number = laragest(&number_list);
//     println!("max_number = {}", max_number);
//     println!("hello world!");
//     let char_list = vec!['a', 'y', 'b'];
//     let max_char = laragest(&char_list);
//     println!("max_chat = {}", &max_char);
// }

// ////--------泛型-----------
// fn laragest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut larger = list[0];
//     for &itme in list.iter() {
//         if itme > larger {
//             larger = itme
//         }
//     }
//     larger
// }

// // 定义结构体中使用泛型
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
// #[derive(Debug)]
// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point { x: 1, y: 2 };
//     println!("integr = {:#?}", integer); // 换行打印

//     let float = Point { x: 1.1, y: 2.2 };
//     println!("float = {:?}", float); // 不换行打印

//     let a = Point2{x:1.1,y:'a'};
//     println!("{:?}",&a)
// }

// // --------枚举中使用泛型-----------
// enum Option<T>{
//     Some(T),
//     None,
// }
// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

//--------在方法里面使用泛型-----------

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    // ipml 是方法
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}
impl<T, U> Point2<T, U> {
    fn creat_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());
    println!("hello world!");
    let p = Point { x: 1.1, y: 2.2 };
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p1 = Point2 { x: 5, y: 2.2 };
    let p2 = Point2{x:"hello",y:'c'};
    let p3 = p1.creat_point(p2);
    println!("x = {}  y = {}", p3.x ,p3.y);
}
