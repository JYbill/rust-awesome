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

// 所有权执行流程
/* fn main() {
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
 */

// 可变引用在同一作用域下只允许存在一个
/* fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s; // r1作用域开始
    // let r2 = &mut s; ❌ 多个可变引用错误
    println!("{}", r1); // r1作用域结束
} */

// 对基本数据进行可变引用，并修改值测试，如果可变引用去引用栈内存，发生值的修改影响到了栈的原始变量，即引用都是引用的指针，是浅拷贝 ✅
fn main() {
    let mut x = 5;
    let y = &mut x; // 引用操作
    *y += 3;
    println!("y {}", *y);
    println!("x {}", x);
}

// 不可变引用、可变引用无法同时使用
/* fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
                 // let r3 = &mut s; // ❌ 不可同时使用
    println!("{}, {}, {}", s, r1, r2);
    // 使用完成后即引用的作用域结束
    let r3 = &mut s;
    println!("r3 {}", r3); // ✅ r3 hello
} */

// 悬垂引用
/* fn main() {
    let _ref_string = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s // ❌ 悬垂指针
    s // ✅权限转移
}
 */
