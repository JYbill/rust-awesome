/**
 * if-else-if
 */
#[allow(unused_variables)]
/* fn main() {
    let flag = 10;
    let num = if flag == 1 {
        1
    } else if flag == 2 {
        2
    } else {
        3
    };
    println!("num = {}", num);
} */

/**
 * for循环
 */
/* fn main() {
    // 输出1 ~ 5
    /* for i in 1..=5 {
        println!("{}", i);
    } */

    // 1 ~ 5
    /* for item in [1, 2, 3, 4, 5] {
        println!("{}", item);
    } */

    // 修改引用
    /* let mut arr = [1, 3, 5, 7, 9];
    for item in &mut arr {
        *item += 1;
    }
    println!("arr: {:?}", arr); // arr: [2, 4, 6, 8, 10] */

    // 迭代器
    /* let arr = [1, 3, 5, 7, 9];
    for (index, val) in arr.iter().enumerate() {
        println!("index: {}, value: {}", index, val); // index: 0, value: 1...
    } */

    // 执行n次
    /* for _ in 0..3 {
        println!("执行3次"); // 该行执行三次
    } */
} */

/**
 * while
 */
/* fn main() {
    let mut num = 0;
    let res = while num <= 5 {
        num += 1;
    };
    println!("num: {}", num); // num: 6
} */

/**
 * loop
 */
/* fn main() {
    let mut num = 1;
    let res = loop {
        if num >= 5 {
            break num;
        }
        num += 1;
    };
    println!("res: {}", res); // res: 5
} */

/**
 * label
 */
#[allow(unused_labels)]
fn main() {
    let mut count = 0;
    let res = 'outer: loop {
        'inner: loop {
            if count >= 20 {
                break 'inner; // 这只会跳出 inner1 循环，这里使用`break`也是一样的
            }
            count += 10;
        }

        'quit: loop {
            break 'outer 100;
        }
    };
    assert!(res == 100);
}
