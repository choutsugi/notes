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
