// use modA::A;
use modA::A as A1;


mod modA {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }
        pub fn print_a(&self) {
            println!("number:{}, name:{}", self.number, self.name);
        }
    }

   pub  mod modB {
        pub fn print_B() {
            println!("B");
        }

       pub  mod modC {
            pub fn print_C() {
                println!("C");
                super::print_B(); // 相对的路径 调用父模块的使用
            }
        }
    }
}

fn main() {
    // let a = modA::A::new_a(); //绝对路径方法
    // let a = A::new_a();
    let a = A1::new_a();
    a.print_a();

    modA::modB::modC::print_C();


    let number = a.number;
    // let name = a.name; // 不能使用私有的


    println!("hello world !")
}
