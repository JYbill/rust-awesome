// 整数数值方法
/* fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(2);
    println!("{}", b); // 1
} */

// 浮点类型
/* fn main() {
    let x = 1.0;
    let y = 2.0;
    println!("x is {}", x);
    println!("y is {}", y);
} */

// 浮点类型陷阱
/* fn main() {
    println!("{}", (0.1_f64 + 0.2 - 0.3).abs() < 0.1)
} */

// 浮点类型NAN
/* fn main() {
    let x = (-42.0_f32).sqrt();
    println!("x is {}", x); // x is NAN
    let is_nan = x.is_nan();
    println!("x is NAN {}", is_nan); // x is NAN: true
} */

// 浮点数上使用方法
fn main() {
    println!("{}", 13.14_f32.round()); // 13
}
