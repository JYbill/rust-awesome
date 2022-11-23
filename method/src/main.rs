/**
 * 为结构体实现方法
 */
/* fn main() {
    struct Rectangle {
        w: i32,
        h: i32,
    }
    impl Rectangle {
        fn new(w: i32, h: i32) -> Rectangle {
            Rectangle { w, h }
        }

        fn area(&self) -> i32 {
            self.w * self.h
        }
    }

    let rect = Rectangle::new(20, 10);
    println!("长方形面积: {}", rect.area());
}
 */

/**
 * 为枚举实现方法
 */
fn main() {
    #[derive(Debug)]
    enum Msg {
        World(String),
    }
    impl Msg {
        fn say(&self) {
            println!("{:?}", self);
        }
    }
    let msg = Msg::World(String::from("aka. xiaoqinvar"));
    msg.say();
}
