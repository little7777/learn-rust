fn main() {
    let rect1 = Rectangle{width: 80, height: 50};
    let rect2 = Rectangle{width: 40, height: 20};
    let rect3 = Rectangle{width: 100, height: 10};

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

struct Rectangle {
    width: u32,
    height: u32,
}
// method 定义, self 是struct Rectangle 的实例

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

