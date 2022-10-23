struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 实例方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数：不与任何实例关联
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
