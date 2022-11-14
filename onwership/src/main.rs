// 触发所有权转移
/* fn main() {
    let s1 = String::from("hello");
    // 发生所有权转移，s1的所有权移动给s2，此时s1不再具有所有权
    // 也就是说，String复合类型的所有权只在s2手中，s1没有了
    let s2 = s1;
    println!("{}, world!", s1); // ❌
}
 */

// 不会触发所有权转移
/* fn main() {
    let s1: &str = "hello";
    let s2 = s1;
    println!("{}, world!", s1); // hello, world!
    println!("{}, world!", s2); // hello, world!
} */

// 深拷贝
/* fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1.push_str(" rust");
    println!("s1 = {}, s2 = {}", s1, s2);
} */

// Copy特征
/* fn main() {
    let s1: &str = "hello";
    let s2: &str = s1;
    println!("{}, {}", s1, s2);
} */

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s指针不再有效，清理了栈内存
    let x = 5; // x 进入作用域
    makes_copy(x);
    // Copy特性仍然可以使用x的值
} // 先清理x的栈内存 -> s因为移除了指针，所以没有任何操作

fn takes_ownership(some_string: String) {
    // some_string得到s的指针
    println!("{}", some_string);
} // some_string调用`drop`方法清理占用的堆内存

fn makes_copy(some_integer: i32) {
    // Copy特征值的拷贝
    println!("{}", some_integer);
} // some_integer销毁清理占用的栈内存
