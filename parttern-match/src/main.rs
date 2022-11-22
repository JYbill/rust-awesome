/**
 * 模式匹配
 */
#[allow(unused_variables, dead_code)]
/* enum Direction {
    North,
    South,
    West,
    East,
}
fn main() {
    let direction = Direction::West;
    let res = match direction {
        Direction::East => "东",
        Direction::North | Direction::South => "北 or 南",
        _ => "西",
    };
    println!("res: {}", res); // res: 西
} */

// match获取枚举内容
/* enum Direction {
    North(String),
    South,
    West,
    East,
}
fn main() {
    let north = Direction::North("北方".to_string());
    match north {
        Direction::North(str) => {
            println!("方向: {}", str);
        }
        _ => {
            println!("nothing");
        }
    }
}
 */

/**
 * if let匹配
 */
/* fn main() {
    let some = Some(3);
    if let Some(3) = some {
        println!("存在Some(3)");
    };
} */

/**
 * 变量覆盖
 */
/* fn main() {
    let some = Some(3);
    if let Some(item) = some {
        println!("存在Some({})", item + 1);
    };
    println!("some = {:?}", some)
}
 */

/**
 * 解构Option
 */
/* fn main() {
    // let num: Option<i32> = Some(1);
    let num: Option<i32> = None;
    let res = match num {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("res = {:?}", res); // res = None
} */

/**
 * while let
 */
/* fn main() {
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素, 返回Option<T>
    while let Some(top) = stack.pop() {
        println!("{}", top); // 3 2 1
    }
} */

/**
 * 序列匹配, 基本类型仅字符串、数字可用，因为它们是连续的
 */
fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
