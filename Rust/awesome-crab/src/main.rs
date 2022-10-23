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
