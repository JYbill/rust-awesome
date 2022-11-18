/*
  tuple元组解构
*/
/* fn main() {
    let tup: (i32, f64, u8) = (1, 1.1, 255);
    let (first, second, third) = tup;
    println!("{first}, {second}, {third}");
} */

/*
  元组通过.访问
*/
/* fn main() {
    let tup: (i32, f64, u8) = (1, 1.1, 255);
    println!("{}", tup.0); // 1
    println!("{}", tup.1); // 1.1
    println!("{}", tup.2); // 255
} */

/*
 元组函数的应用场景
*/
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
