// 字符类型
/* fn main() {
    let x = '🥶';
    println!("rust字符占用字节大小：{}字节", std::mem::size_of_val(&x));
}
 */

// 布尔类型
/* fn main() {
    let foo = true;
    if foo {
        println!("true");
    }
} */

// 单元类型
/* fn main() {
    let unit: () = ();
    assert_eq!(ret_fn(), unit);
}
fn ret_fn() {} */

// 表达式
fn main() {
    let foo = {
        let _x = 1;
        1 + 1
    };
    println!("foo is {}", foo); // foo is 2
    assert_eq!(test(), ()); // ✅
}
fn test() {}
