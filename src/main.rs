mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator!");
        }
    }
    pub mod produce_washing_machine {
        // 洗衣机
        pub fn produce_washing_maching() {
            println!("produce washing machine !");
        }
    }
}


fn main() {
    // 调用模块使用
    factory::produce_refrigerator::produce_re();
    factory::produce_washing_machine::produce_washing_maching();
    println!("hello world !")
}


