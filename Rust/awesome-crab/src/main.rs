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
