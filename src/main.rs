mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator!");
        }
    }

    mod produce_washing_machine {
        // 洗衣机
        fn produce_washing_maching() {
            println!("produce washing machine !");
        }
    }
}


fn main() {
    // 调用模块使用
  factory::produce_refrigerator::produce_re();

    println!("hello world !")
}


