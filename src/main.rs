//1、trait用于定义与其它类型共享的功能，类似于其它语言中的接口。
//（1）可以通过trait以抽象的方式定义共享的行为。
//（2）可以使用trait bounds指定泛型是任何拥有特定行为的类型。
//2、定义trait
pub trait GeInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}
/**/
// 3 实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GeInformation for Student {
    fn get_name(&self) -> &String {
      &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GeInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

//4 默认实现：可以在定义trait的时候提供默认的行为， trait的类型可以使用默认的行为
// 5 trait作为参数
fn print_information(item: impl GeInformation){
    println!("name = {}",item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let s = Student{ name:"xiaohong".to_string(), age: 10};
    let t = Teacher{ name: "xiaohuo".to_string(), age: 10 ,subject:String::from("hello")};
    // println!("student ,nam= {}, age={}",s.get_name(),s.get_age());
    // println!("Teacher ,nam= {}, age={},subject={}",t.get_name(),t.get_age(),t.subject);
    print_information(s);
    print_information(t);
    println!("hello world!")
}
