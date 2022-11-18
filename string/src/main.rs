/*
  字符串操作
*/
/* fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // h~o 左闭右开
    let world = &s[6..11]; // w~d
    println!("{}", hello); // hello
    println!("{}", world); // world
} */

/*
  中文切片的陷阱
*/
/* fn main() {
    let s = String::from("小青蛙");
    let first = &s[0..3];
    let err = &s[3..5]; // ❌ 编译不通过，青在`小青蛙`应该是3..6
    let last = &s[6..];
    println!("{}", first); // 小
    println!("错误演示 {}", err); // 小
    println!("{}", last); // 蛙
}
 */

/*
 数组切片
*/
/* fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]); // ✅ 引用指向同一个数组(虽然起始位置不同)，比较引用通过
} */
/*
  String、&str互转
*/
#[allow(unused)]
/* fn main() {
    // &str -> String
    let s1 = String::from("hello");
    let s2 = "hello".to_string();

    // String -> &str
    let test = String::from("test");
    let s3 = test.as_str(); // &str
    let s4 = &test; // &String
    let s4 = &test[..]; // &str
} */

/*
  替换所有
*/
/* fn main() {
    // let s1 = String::from("hello, hello, hi");
    // let s2 = s1.replace("hello", "ha");
    // println!("{}", s2); // ha, ha, hi
    // let s2 = s1.replacen("hello", "ha", 1);
    // println!("{}", s2); // ha, hello, hi

    let mut s1 = String::from("hello, hello, hi");
    s1.replace_range(0..5, "replace");
    println!("{}", s1); // replace, hello, hi
} */

/*
  删除
*/
/* fn main() {
    // let mut s = String::from("rust");
    // let c = s.pop();
    // dbg!(c); // Some('t')

    // let mut s = String::from("你好");
    // let c = s.remove(0);
    // // let c = s.remove(1); // ❌ 索引为1的字符无法分割，它在你的中间 [11, 22(here), 33]
    // println!("char is {c}"); // char is 你
    // println!("{}", s); // 好

    let mut s = String::from("你好");
    let c = s.remove(3);
    println!("{}", s); // 你
}
 */

/*
 拼接
*/
/* fn main() {
    // let s = String::from("hello");
    // let source = " rust!";
    // let res = s + source;
    // println!("{res}"); // hello rust!

    let s = String::from("hello");
    let res = format!("{}-{}", s, "Rust");
    println!("{res}"); // hello-Rust
    println!("{s}"); // hello
} */

/*
  转译
*/
/* fn main() {
    // println!("\u{211D}"); // ℝ
    // println!(
    //     "测试
    // 换行
    // 不换行\
    // 不换行测试"
    // )

    // 不转译
    println!("{}", "hello \\x52\\x75\\x73\\x74"); // hello \x52\x75\x73\x74
    println!("{}", r#""包括双引号\\x52""#); // "包括双引号\\x52"
    println!("{}", r###""包括双引号和#号\\x52"#"###); // "包括双引号和#号\\x52"#
} */

// 操作utf8字符
/* fn main() {
    for c in "哈喽".bytes() {
        println!("{c}")
    }
    // 229
    // 147
    // 136
    // 229
    // 150
    // 189
}
 */

/*
 utf8字节数组
*/
fn main() {
    let bytes: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytes);
    // A byte string: [116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103]
}
