/**
 * 范型函数
 */
/* fn main() {
    println!("add = {}", add(1.1, 2.3)); // 3.4
}
fn add<T: std::ops::Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}
 */

/**
 * 范型结构体
 */
/* #[allow(dead_code, unused_variables)]
fn main() {
    #[derive(Debug)]
    struct RGB<T> {
        red: T,
        yellow: T,
        blue: T,
    }
    let white = RGB {
        red: 255,
        yellow: 255,
        blue: 255,
    };
    println!("white rgb: {:?}", white); // white rgb: RGB { red: 255, yellow: 255, blue: 255 }
} */

/**
 * 专属范型方法
 */
/* fn main() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<String> {
        fn msg(&self) -> String {
            let res = String::from("") + &self.x + ", " + &self.y;
            return res;
        }
    }

    let p = Point {
        x: "10".to_string(),
        y: "20".to_string(),
    };
    println!("p: {}", p.msg()); // p: 10, 20
                                // let p2 = Point { x: 1, y: 2 };
                                // println!("p: {}", p2.msg()); // ❌ 没有msg方法
} */

/**
 * const范型值
 */
fn main() {
    print([1, 2, 3]); // [1, 2, 3]
    print([1.1, 2.2]); // [1.1, 2.2, 3.3]
                       // errPrint([1, 2]) // ❌ 长度与方法不一致
}

// const N: usize它用来替代的是数组的长度
fn print<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// ❌ 长度也是类型的一部分
fn errPrint<T: std::fmt::Debug>(arr: [T; 3]) {
    println!("{:?}", arr);
}
