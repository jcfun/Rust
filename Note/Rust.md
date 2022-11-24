# Rust

## 1、Rust简介

```rust
println!("hello world") // 打印文本
```

+ rust的缩进是4个空格，而不是tab
+ println!是一个rust macro，如果是函数的话，就没有!
+ rust文件命名规则为下划线命名法
+ cargo build [--release]
  + 编译rust源代码并生成可执行文件，--release会在编译时对代码进行优化
+ cargo check
  + 检查代码，确保代码可以通过编译，但是不生成可执行文件
+ cargo run
  + 编译rust源代码并执行可执行文件，如果已经对源代码进行过编译且没有对源代码进行修改，这会直接跳过编译，执行可执行文件

## 2、猜数游戏

```rust
use std::io;
use rand::Rng; // trait
// use rand::thread_rng;

fn main() {
    println!("猜数！");
    let secret_number = rand::thread_rng().gen_range(1.. 101);
    println!("神秘数字是：{}", secret_number);

    println!("猜测一个数");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是：{}", guess);
}
```



+ 一般情况下，rust会将prelude模块导入到每一个程序的作用域中
+ 使用use关键字导入需要的库，例如：`use std::io;`
+ let关键字用来声明一个变量，但是这个变量默认是不可变的，如果想要修改这个变量，需要在let后面加上mut（mutable）关键字，例如：`let mut a = 1;`
+ String::new()将返回一个空白的字符串，new()是String类型的关联函数，类似于Java中的静态方法
+ io::stdin()将返回一个Stdin类型的实例，该实例会被用作句柄（handle）用来处理终端中的输入
+ &mut表示获取guess的引用，rust中引用默认同样是不可变得，因此同样需要加上mut
+ read_line方法会返回io::Result\<usize\>类型，这个类型是一个enums类型，其共有两个值：Ok、Err，当程序发生异常时，会调用expect方法，并将其中的字符串显示给用户
+ rand::Rng是一个trait，类似于Java中的接口，通过rand::thread_rng()返回Rng的实现ThreadRng类型