## 一、开发环境

### 1.1 安装编译器

安装：

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

卸载：

```bash
rustup self uninstall
```

检查安装：

```bash
rustc --version
cargo --version
```

### 1.2 开发工具

开发工具：VS Code。

插件：

- rust-analyzer
- crates
- Even Better TOML：.toml文件支持
- Error Lens：错误提示
- One Dark Pro：主题
- CodeLLDB：Debugger程序

自动格式化配置（Ctrl + Shift + P）：

```json
{
    "editor.unicodeHighlight.nonBasicASCII": false,
    "workbench.colorTheme": "One Dark Pro Darker",
    "editor.fontSize": 18,
    // "editor.fontFamily": "Fira Code, Consolas, 'Courier New', monospace",
    "editor.fontFamily": "Fira Code Light, Consolas, Microsoft YaHei",
    "editor.fontLigatures": true,
    "debug.console.fontSize": 18,
    "debug.console.fontFamily": "Fira Code Light, Consolas, Microsoft YaHei",
    "terminal.integrated.fontFamily": "Fira Code Light, Consolas, Microsoft YaHei",
    "window.zoomLevel": 1.2,
    "remote.SSH.remotePlatform": {
        "45.136.15.240": "linux"
    },
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    },
}
```

### 1.3 配置镜像源

新增配置文件（.cargo/config），放置于用户目录或项目根目录下，以用户目录为例：

```
$HOME/.cargo/config
```

配置内容：

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'tuna' # 如：tuna、sjtu、ustc，或者 rustcc

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```

## 二、猜数字游戏

新建项目：

```bash
cargo new guessing_game
```

添加依赖：Cargo.toml

```toml
# ...
[dependencies]
rand = "0.8.5"  	# 随机数生成
colored = "2.0.0" 	# 标准输出颜色
```

代码：main.rs

```rust
use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    
    println!("猜数字游戏1.0");

    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("秘密数字是：{}", secret_number);

    loop {
        println!("请输入一个数字：");
        let mut guess = String::new();
        // 读取标准输入
        io::stdin()
            .read_line(&mut guess)
            .expect("读取用户输入错误！");

        // 变量遮蔽（shadowing）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("你输入的数字是：{}", guess);

        // 模式匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "太小了".red()),
            Ordering::Equal => {
                println!("{}", "你赢了".green());
                break;
            }
            Ordering::Greater => println!("{}", "太大了".red()),
        }
    }
}
```

## 三、基础概念

### 3.1 变量

rust变量默认不可变，若需修改，可通过`mut`关键字指定为可变变量。

```rust
fn main() {
    
    let mut x = 5;
    println!("x的值是：{}", x);

    x = 10;
    println!("x的值是：{}", x);
}
```

### 3.2 常量

rust中常量类型需要显式指定；常量名按惯例使用大写，多个单词使用下划线连接。

```rust
fn main() {
    
    const SUBSCRIBER_COUNT: u32 = 100_000;

    println!("SUBSCRIBER_COUNT = {}", SUBSCRIBER_COUNT);
}
```

### 3.3 变量遮蔽

rust中允许重新声明变量且可以改变原有类型，被遮蔽的原变量失效。

```rust
fn main() {
    
    let x = 6;
    println!("x的值是：{}", x);

    let x = "Six";
    println!("x的值是：{}", x);
}
```

### 3.4 标量类型

**整数**

```rust
fn main() {
    
    // 整数（integers）
    let a = 98_222; // 十进制
    let b = 0xff; // 十六进制
    let c = 0o77; // 八进制
    let d = 0b1111_0000; // 二进制
    let e = b'A'; // 字节（u8）
    println!("{} {} {} {} {}", a, b, c, d, e);
}
```

**浮点数**

```rust
fn main() {
    
    // 浮点数（floating point numbers）
    let f = 2.0; // 浮点数缺省为f64
    let g: f32 = 3.0;
    println!("{} {}", f, g);
}
```

**布尔**

```rust
fn main() {
    
    // 布尔（booleans）
    let h = true;
    let i = false;
    println!("{} {}", h, i);
}
```

**字符**

```rust
fn main() {
    
    // 字符（characters）：unicode字符
    let j = 'z';
    let k = 'ʣ';
    let l = '😎';
    println!("{} {} {}", j, k, l);
}
```

### 3.5 复合类型

**元组**

```rust
fn main() {
    
    // 元组（tuple）
    let tup = ("tsugi", 100_100);

    // 解构元组
    let (name, balance) = tup;
    println!("{} {}", name, balance);

    // 按索引获取元组数据：下表从0开始
    let name = tup.0;
    let balance = tup.1;
    println!("{} {}", name, balance);
}
```

**数组**

```rust
fn main() {
    
    // 数组（array）
    let error_codes = ['😛', '😥', '😵'];
    let not_found = error_codes[1];
    println!("{} not found", not_found);

    // 快速创建数组：创建具有8个元素的数组，使用0填充。
    let byte = [0; 8];

    // 数据越界，运行时错误。
    let x = byte[byte.len() + 1];
    println!("x = {}", x);
}
```

### 3.6 方法

Rust代码分为语句和表达式，函数中最后一句为表达式则隐式地做为返回值返回。

```rust
fn main() {
    
    let sum = add(1, 2);
    println!("The sum is: {}", sum)
}

// 方法（function）
fn add(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // 函数中最后一个表达式的值隐式返回。
    x + y
}
```

### 3.7 控制流

**if/else**

```rust
fn main() {
    
    // 控制流 if/else
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }
}
```

**if/else in let**

```rust
fn main() {
    
    // 控制流 if/else in let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);
}
```

**loop**

```rust
fn main() {
    
    // 控制流 loop
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    println!("The counter is {}", counter);
}
```

**while**

```rust
fn main() {
    
    // 控制流 loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1
    }

    println!("起飞！！！");
}
```

**for in**

```rust
fn main() {
    
    // 控制流 for in
    let arr = [10, 20, 30, 40, 50];

    // 迭代器
    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }
}
```

### 3.8 注释

```rust
fn main() {

    // 单行注释

    /*
        块注释
    */
}
```

## 四、所有权

### 4.1 所有权规则

如下：

1. Rust中每一个值都存在与之对应的成为所有者（Owner）的变量。
2. 同一时间点，一个值只能有一个所有者。
3. 当所有者推出作用域后，对应的值也将被丢弃（销毁）。

### 4.2 数据分配规则

- 编译时可确定大小并且大小不变的数据存放在栈上，如integer、reference、字符串字面量等。
- 编译时大小不确定的数据存放在堆上，如String、Vector。
- 栈的访问性能优于堆。

### 4.3 拷贝与移动

实现了Copy Trait的类型在赋值时执行拷贝；未实现Copy Trait的类型在赋值时将移交所有权（移动）。

```rust
fn main() {

    let x = 7;
    let y = x; // Copy：integer/boolean/character类型实现了Copy Trait，不会转移所有权。
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // Move：所有权转移，s1失效。
    println!("{} world!", s2)
}
```

### 4.4 引用与借用

变量传入函数时将丧失所有权：

```rust
fn main() {

    let str = String::from("hello");

    giving_ownership(str); // 所有权移入函数。
    // println!("str = {}", str); // 已丧失所有权，无法再使用。
}

fn giving_ownership(string: String) {
    println!("received param: {}", string);
}
```

为避免所有权转移，可使用**引用**作为参数（**创建引用的过程称为借用**）。

```rust
fn main() {
    
    let str = String::from("hello");

    giving_ownership(&str); // 引用不转移所有权。
    println!("str = {}", str);
}

fn giving_ownership(string: &String) {
    println!("received param: {}", string);
}
```

### 4.5 引用规则

**规则一：特定作用域内，对于某个特定的数据，只能有一个可变引用（避免数据竞争）。**

```rust
fn main() {
    
    let mut str = String::from("hello");

    let r1 = &mut str;
    // let r2 = &mut str; // 不可同时存在多个可变引用。

    println!("r1 = {}", r1);
    // println!("r2 = {}", r2);
}
```

**规则二：特定作用域内，对于某个特定的数据，如果已存在不可变引用，则无法再添加可变引用。**

```rust
fn main() {
    
    let mut str = String::from("hello");

    let r1 = &str;
    // let r2 = &mut str; // 不可同时存在可变引用与不可变引用。

    println!("r1 = {}", r1);
    // println!("r2 = {}", r2);
}
```

### 4.6 悬垂引用

```rust
fn main() {
    
    // dangle();
}

// 悬垂引用：返回引用，但引用对象超出作用域后已销毁。
// fn dangle() -> &String {
//     &String::from("hello")
// }
```

## 五、切片

使用示例：获取第一个单词。

```rust
fn main() {
    let str = "hello world";

    let word = first_word(str);
    println!("The first word is {}", word);
}

fn first_word(str: &str) -> &str {
    // 字符串转为切片
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
```

## 六、结构体

### 6.1 单元结构体

```rust
// 定义单元结构体
struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

fn main() {
    let tsugi = build_user(String::from("tsugi@gmail.com"), String::from("tsugi"));

    // 使用已有结构体变量构建新的结构体变量
    let jerry = User {
        email: String::from("jerry@gmail.com"),
        username: String::from("jerry"),
        ..tsugi
    };
    
}

// 通过函数创建结构体变量
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 6.2 元组结构体

```rust
// 定义元组结构体
struct Color(i32, i32, i32);

fn main() {
    let red = Color(255, 0, 0);

    // 访问元组结构体成员
    println!("red({},{},{})", red.0, red.1, red.2);
}
```

### 6.3 结构体打印

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    // 打印结构体（#美化输出）
    println!("rect: {:#?}", rect);
}
```

> 为结构体启用派生Debug宏。

### 6.4 实例方法与关联函数

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现块：一个结构体可以有多个实现块。
impl Rectangle {
    // 实例方法：必须绑定实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数：不绑定任何实例
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(20);

    println!("矩形面积为{}平方像素。", rect.area());
}
```

## 七、枚举与模式匹配

### 7.1 枚举定义

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // ...
}
```

### 7.2 枚举存储不同类型

```rust
enum Message {
    Quit,                       // 不存放数据
    Move { x: i32, y: i32 },    // 存放匿名结构体
    Write(String),              // 存放字符串
    ChangeColor(i32, i32, i32), // 存放三个整数
}
```

### 7.3 关联函数

```rust
// enum Message...

impl Message {
    fn some_func() {
        println!("awesome rust enum")
    }
}

fn main() {
    Message::some_func();
}
```

### 7.4 可选枚举

可选枚举Option：

```rust
enum Option<T> {
    Some(T), // 有值
    None,    // 无值
}

fn main() {
    let some_number = Some(6);
    let some_string = Some("string");
    let absent_number: Option<i32> = None; // 可选枚举初始化为None时需要显式标注类型
}
```

使用：

```rust
fn main() {
    let x = 4;
    let y = Some(4);
    let sum = x + y.unwrap_or(0); // 获取枚举值，缺省值为0

    println!("sum = {}", sum);
}
```

### 7.5 模式匹配

**枚举与模式匹配**

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state = {:?}", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

**可选枚举与模式匹配**

```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // ...
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // 返回值包含在Some中。
    }
}
```

**可忽略的枚举值处理简化**

```rust
fn main() {
    let some_value = Some(2);

    match some_value {
        Some(2) => println!("two"),
        _ => (), // 可忽略枚举值简化处理，无需一一列举。
    }
    // 以上模式匹配可使用if-let简化：
    if let Some(2) = some_value {
        println!("two");
    }
}
```

## 八、模块系统
### 8.1 模块系统组成
- 工作区包含若干个相互关联的Package。
- 每个Package（通过cargo new创建）包含若干个Crate（Binary或Library）。
- 每个Crate包含若干个Module（用于控制访问权限）。
- 每个Module包含若干个源文件。

### 8.2 Crate 
**Crate规则**

- 规则一：一个包中必须至少包含一个crate。
- 规则二：一个包中可以不包含library crate或包含一个library crate。
- 规则三：一个包中可以包含任意数量的binary crate。

> 默认src目录下main.rs为binary crate root，lib.rs为library crate root。

**Crate创建**

创建binary crate：
```bash
cargo new awesome-crab
```
创建library crate：
```bash
cargo new --lib awesome-crab-lib
```

### 8.3 Module

**Module定义**

```rust
// 模块定义模块
mod front_of_house {
    // 嵌套的内部模块
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

**Module使用**

使用pub关键字定义公开模块及方法。

```rust
mod front_of_house {
    // 使用pub公开模块
    pub mod hosting {
        pub fn add_to_waitlist() {} // 使用pub公开方法
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

使用super关键字访问父级模块方法：

```rust
fn serve_order() {}
mod back_of_house {
    fn fix_incorret_order() {
        cook_order();
        // 使用super访问父级模块中的方法。
        super::serve_order();
    }

    fn cook_order() {}
}
```

模块内结构体及其字段默认缺省为私有：

```rust
mod back_of_house {

    pub struct Breakfast {
        pub toast: String,	// 使用pub公开后可访问
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("梨"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("黑麦");
    meal.toast = String::from("小麦");
}
```

模块内枚举值无需一一指定公开：

```rust
mod back_of_house {
    // 只需指定枚举公开
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

使用use简化module引入：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/**
 * 使用user引入模块
 * 惯例：对于函数引入其父级模块，对于结构体、枚举则直接引入。
 */
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

解决同名引入冲突：

```rust
// 引入父级以解决冲突
// use std::fmt;
// use std::io;

// 使用as定义别名解决冲突
use std::fmt::Result;
use std::io::Result as IoResult;

/**
 * 解决同名冲突：
 *  方式一：引入父级
 *  方式二：使用as定义引入别名
 */

fn func1() -> Result {
    Ok(())
}

fn func2() -> IoResult<()> {
    Ok(())
}
```

引入并导出模块：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 引入并导出模块
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {}
```

批量引入：

```rust
// 批量引入
use rand::{CryptoRng, ErrorKind::Transient, Rng};
// 嵌套引入
use std::{self, write};
// 全量引入
// use std::io::*;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 100);
}
```

模块实现与声明分离：

```rust
// 模块声明：实现在与模块同名的文件中。
mod front_of_house;

// 实现：front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

模块及子模块分离：

- lib.rs文件：声明front_of_house模块，实现在同名文件中。
- front_of_house.rs文件：front_of_house模块的实现，在其中声明hosting子模块。
- front_of_house文件夹：映射front_of_house父模块。
  - hosting.rs文件：hosting子模块的实现。

## 九、集合

### 9.1 Vector

基本使用：

```rust
fn main() {
    // 创建不含任何元素的vector
    let mut v1: Vec<i32> = Vec::new();
    // vector添加元素
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // 使用vec宏创建vector
    let mut v2 = vec![1, 2, 3];

    // 使用索引访问vector元素：vector内存动态分配，运行时使用索引可能发生越界
    println!("The third element of v1 is {}", &v1[2]);
    println!("The third element of v2 is {}", &v2[2]);

    // 使用get获取vector元素
    match v1.get(2) {
        Some(value) => println!("The third element of v1 is {}", value),
        None => println!("There is no third element in v1."),
    }

    // ERR：可变引用与不可变引用不可同时存在。
    // let third = &v2[1]; // 不可变引用
    // v2.push(4); //可变引用

    // 遍历
    for i in &mut v2 {
        *i += 10;
    }
    for i in &v2 {
        println!("{}", i);
    }

    // 定义枚举值为不同类型的枚举
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 使用枚举以实现vector存储不同类型数据
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an iteger!"),
    };
}
```

### 9.2 String

String为使用UTF-8编码的字节集合。

```rust
fn main() {
    let s1 = String::new(); // 创建空字符串
    let s2 = "initial contents"; // 创建字符串切片
    let s3 = s2.to_string(); // 字符串切片转字符串
    let s4 = String::from("initial contents");

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('!');

    let s6 = String::from("Hello, ");
    let s7 = String::from("World!");
    let s8 = s6 + &s7; // s6所有权转移到s8
    let s9 = format!("{}{}", s8, s8); // format!不转移所有权
    println!("{} {}", s7, s8);

    // 不支持通过索引获取字符串中字符（变长编码）。
}
```

> 可通过字符串切片的chars()方法遍历所有字符。

### 9.3 HashMap

基本使用：

```rust
use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // insert传入所有权
    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    // 遍历
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    // 插入相同值则覆盖
    scores.insert(String::from("Green"), 50);
    scores.insert(String::from("Green"), 60);

    // 若不存在则插入
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(90); // 将忽略

}
```

统计单词数量：

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
```

## 十、错误处理

### 10.1 Result枚举与可恢复的错误

**使用match处理错误**

```rust
use std::{fs::File, io::ErrorKind};

fn main() {
    // 使用match处理错误
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("无法创建文件：{:?}", e),
            },
            other_error => {
                panic!("无法打开文件：{:?}", other_error)
            }
        },
    };
}
```

**使用闭包处理错误**

```rust
use std::{fs::File, io::ErrorKind};

fn main() {
    // 使用闭包处理错误
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("无法创建文件：{:?}", error);
            })
        } else {
            panic!("无法打开文件：{:?}", error);
        }
    });
}
```

**使用unwrap处理错误**

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

**使用expect处理错误**

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("无法打开文件：hello.txt");
}
```

### 10.2 错误传播

**普通函数错误传播**

```rust
use std::fs::{self, File};
use std::{self, io};

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // // 使用?简化
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // // 使用方法级联简化
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // 再简化
    fs::read_to_string("hello.txt")
}

fn main() {
    read_username_from_file();
}
```

**main函数错误传播**

```rust
use std::error::Error;
use std::fs::{self, File};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

### 10.3 panic与不可恢复的错误

panic宏使用场景：

- 不可恢复的错误
- 样例代码
- 原型代码
- 测试代码

> 使用RUST_BACKTRACE=1在程序panic时输出调用栈。

## 十一、泛型

### 11.1 泛型方法

```rust
fn main() {
    let number_list = vec![34, 35, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("最大数字为：{}", largest);
}

// <T: PartialOrd> 限制传入类型必须实现PartialOrd与Copy trait
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
```

### 11.2 泛型结构体

```rust
// 泛型结构体
struct Point<T, U> {
    x: T,
    y: U,
}

// 结构体实现块
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只适用于f64类型
impl Point<f64, f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}", p.x())
}
```

### 11.3 泛型枚举

Option与Result：

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 十二、trait

trait，特征，用于定义共享行为（接口）。

### 12.1 trait定义

```rust
// 定义trait
pub trait Summary {
    // fn summarize(&self) -> String;

    // 缺省实现
    fn summarize(&self) -> String {
        // 缺省实现中调用其他方法
        format!("Read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}
```

### 12.2 为结构体实现trait

```rust
pub struct Article {
    pub author: String,
    pub content: String,
    pub headline: String,
}

// 为Article实现Summary trait
impl Summary for Article {
    // summarize使用缺省实现

    // 覆写summarize_author
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为Tweet实现Summary trait
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 覆写缺省实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### 12.3 trait作为函数参数

trait作为函数参数：

```rust
// 方式一
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 方式二
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

> 当参数为多个且类型相同时使用方式二。

限制函数参数实现多个trait：

```rust
// 方式一
pub fn notify(item: &impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}
// 方式二（使用泛型）
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

限制函数泛型参数实现多个trait：

```rust
fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
// 以上可优化为👇
fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
```

### 12.4 trait作为函数返回值

***注：返回类型必须在编译时可确定，不可使用else返回不同类型！***

```rust
fn returns_summarizable() -> impl Summary {
    // Tweet实现了Summary
    Tweet {
        username: String::from("tsugi"),
        content: String::from("bad bye!"),
        reply: false,
        retweet: false,
    }
}
```

### 12.5 使用trait限制有条件的使用方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 仅实现了Display和PartialOrd的类型可调用cmp_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### 12.6 通用实现

即为实现了某个trait的类型实现另外的trait。

```rust
// 为实现了Display的类型再添加ToString实现
impl<T: Display> ToString for T {}
```

## 十三、生命周期

### 13.1 悬空引用

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x; // 悬空引用：x出作用域后回收。
    }

    println!("r: {}", r);
}
```

### 13.2 生存期注解

函数返回值生存期注解：

```rust
fn main() {
    let str1 = String::from("abcd");

    {
        // str2生存期小于str1
        let str2 = String::from("1234");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", result);
    }
}

// 借用检查器报错（无法推测返回值生存期），需要通过生存期注解（'a）指定返回值生存期。
// 生存期注解不改变生存期。
// 以下函数在使用生存期注解后返回值的生存期取参数x与y中生存期最小的一方。
fn longest<'a>(x: &'a str, y: &'a str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// & i32        // 引用。
// &'a i32      // 具有显式生存期的引用。
// &'a mut i32  // 具有显式生存期的可变引用。
```

> 使用注解帮助借用检查器发现悬挂指针🙏！

结构体生存期注解：

```rust
struct ImportantExcerpt<'a> {
    // 生存期注解指明：结构体的生存期不能大于其字段part的生存期
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael, Some years ago...");
    let fisrt_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: fisrt_sentence,
    };
}
```

获取第一个单词（生存期注解版）：

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### 13.3 生存期规则

规则：

- 每个引用参数都是一个具有生存期的参数。
- 若仅一个输入生存期参数，则该生存期将赋给所有的输出生存期参数。
- 如多个输入生存期参数中存在&self或&mut self，则self生存期赋给所有输出生存期参数。

栗子：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // fn return_part(&'a self, announcement: &str) -> &'a str {

    // 自动推导生存期
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### 13.4 静态生存期

静态生存期，同程序生存期相同。

```rust
fn main() {
    // 静态生存期
    let s: &'static str = "I have a static lifetime.";
}
```

### 13.5 生存期注解与特征限制的泛型

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
```



