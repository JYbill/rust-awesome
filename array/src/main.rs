/*
  数组u
*/
#[allow(unused_variables)]
/* fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = ["1", "2", "3"];
    let arr3 = [3; 5]; // 长度为5，初始化值都为3
    println!("{:?}", arr3); // [3, 3, 3, 3, 3]

    let arr4 = [1, 2, 3];
    let first = arr4[0];
    let second = arr4[1];
    println!("{first}, {second}"); // 1, 2
} */

/*
  数组越界访问 例子
*/
/* use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
} */

/*
  测试：编译时检测数组越界问题
*/
/* fn main() {
    let arr = [3; 3];
    println!("{}", arr[3]); // 编译时报错，数组长度为3，索引只能取到2
} */

/**
 * 复杂数据类型的填充
 */
/* fn main() {
    let array: [String; 8] = core::array::from_fn(|i| String::from("rust is good!"));
    println!("{:#?}", array);
}
 */

/**
 * 数组切片
 */
/* fn main() {
    let arr = [1, 2, 3];
    let ref_arr1: &[i32] = &arr[0..2];
    let ref_arr2: &[i32; 3] = &arr;
    println!("{:?}", ref_arr1); // [1, 2]
    println!("{:?}", ref_arr2); // [1, 2, 3] 推荐👍
}
 */

/**
 * get方法使用
 */
/* fn main() {
    let mut arr = [1, 2, 3];
    let value = arr.get(2);
    println!("{:?}", value); // Some(3)
    println!("{:?}", arr.get(3)); // None
} */

/**
 * 数组是基本数据类型所以是值的神拷贝
 */
fn main() {
    let mut arr1 = [1, 2, 3];
    let arr2 = arr1;
    arr1[0] = 100;
    println!("{:?} {:?}", arr1, arr2); // [100, 2, 3] [1, 2, 3]
}
/* fn main() {
    let mut arr1 = [String::from("node"), String::from("js")];
    let arr2 = arr1; // 复杂数据类型，发生所有权转移arr1 -〉 arr2
                     // arr1[0] = "ts".to_string(); // ❌编译时错误
    println!("{:?} {:?}", arr1, arr2); // ["ts", "js"] ["node", "js"]
} */
