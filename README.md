# Tobs
### 1 定义
 （1）包： Cargo的一个功能，允许构建、测试和分享

 （2）Crate： 一个模块的树形结构，形成库或二进制项目

 （3）模块： 通过use来使用， 用来控制作用域和路径的私有性

 （4）路径： 一个命名例如结构体，函数或模块等项的方式

### 2 包和Crate
  (1) crate root 是一个源文件， Rust编译器以它为起始点，并构成你的crate的根模块

  (2) 包提供一系列功能的一个或者Crate

  (3) Crate root是src/main.rs或者是src/lib.rs。 说明：如果只有main.rs则说明这个包有一个crate(main)，如果同事拥有main、rs和其他的libs.rs(不一定是这个名字)则说你拥有多个crate

  (4) crate会将衣蛾作用域的相关功能分组到一起，使得该功能可以很方便的多个项目之间共享

  ### 3  使用模块控制作用域和私有性
  (1) 创建一个lib可以通过cargo new --lib libname来进行创建

  (2) 中默认所有项(函数、方法、结构体、枚举、模块和常量) 都是私有，需用使用pub才能暴露给外部
  
  (3) 创建模块 例如：
  ```rust
  //factory.rs
 mod  refrigerator { // 冰箱 // 需要使用pub ，否则别人无法使用
    fn refrigeration() { // 需要使用pub，否则别人无法使用

    }
    
 }
  ```

  (4)