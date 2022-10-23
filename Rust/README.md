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

