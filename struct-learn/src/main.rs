/* struct User {
    username: String,
    age: u8,
    address: String,
    sex: bool,
} */
/*
  结构体定义与初始化
*/
/* fn main() {
    let mut user = User {
        sex: true,
        address: String::from("武汉市"),
        age: 23,
        username: "🐸".to_string(),
    };
    println!("姓名 {}", user.username);
    println!("年龄 {}", user.age);
    // 必须标记为mut可变才允许修改结构体
    user.address = String::from("比奇堡贝壳街");
    println!("地址 {}", user.address);
} */

/*
  结构体解构
*/
/* fn main() {
    let user1 = User {
        sex: true,
        address: String::from("武汉市"),
        age: 23,
        username: "🐸".to_string(),
    };
    let user2 = User {
        username: String::from("小青蛙"),
        ..user1
    };
    println!("{:?} {:?}", user2.username, user2.address);
    // 所有权发生转移过，所以访问user1的address会报错，有Copy特征的则不会
    // println!("{:?}", user1.address); // ❌ 编译时报错
    println!("{:?} {:?}", user1.sex, user1.age);
    println!("{:?}", user1.username); // username 没有使用在user2上，未发生所有权转移
}
 */

/*
 结构体内存编排
*/
/* #[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
} */

/*
  元组结构体
*/
/* #[derive(Debug)]
struct Point2D(i32, i32);
fn main() {
    let p = Point2D(1, 20);
    println!("{:?}", p); // Point2D(1, 20)
}
 */

/*
 结构体输出
*/
/* #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rect1); // Rectangle { width: 30, height: 50 }
    println!("{:#?}", rect1);
    /*
     Rectangle {
         width: 30,
         height: 50,
     }
    */
} */

/*
  元组数组解构
*/
struct Point(i32, i32, i32);
fn main() {
    let p = Point(1, 2, 3);
    #[allow(unused_variables)]
    let Point(a, b, c) = p;
}
