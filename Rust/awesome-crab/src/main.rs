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
