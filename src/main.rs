// use::mylib::factory;
// use::mylib::factory::produce_refrigerator;
// use::mylib::factory::produce_refrigerator::produce_re;
// use mylib::factory::produce_washing_machine as A; // 重命名 简写
use mylib::factory::*;

fn main() {
    mylib::factory::produce_refrigerator::produce_re(); //绝对路径
    // factory::produce_washing_machine::produce_washing_maching();
    produce_washing_machine::produce_washing_maching();
    // A::produce_washing_maching();

    println!("hello world !")
}


