/* #[derive(Debug)]使用 */
/*
// #[derive(Debug)]
struct PrintTest {
    name: String,
}

fn main() {
    let test = PrintTest { name: String::from("test") };
    println!("{:?}, name = {:?}", test, test.name);
}
*/

/* 默认rust会导入std::prelude模块 */
/*use std::convert::TryInto; // 可忽略
fn main() {
    let a: i32 = 100000000;
    // let a: u16 = 100;

    let covert: u16 = a.try_into()
        .unwrap();
    println!("covert = {}", covert);
}
*/

/* 自定义结构体实现+运算符特征： obj1 + obj2 */
/*use std::ops::Add;

#[warn(dead_code)]
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Self) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point {
        x: 10,
        y: 10,
    };
    let p2 = Point {
        x: 5,
        y: 5,
    };
    println!("p1 + p2 = {:?}", p1 + p2);
}*/

/* 自定义实现Display特征 */
use std::fmt::{Display, Formatter, Result};

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point  {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Point: x -> {}, y -> {}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 22, };
    println!("{}", p);
}
