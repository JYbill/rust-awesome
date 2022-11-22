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
/* fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
 */

/**
 * 枚举各种值解构模式匹配
 */
/* enum Color {
    Rgb(u8, u8, u8),
    Hsv(i32, i32, i32),
}
enum Msg {
    Quit,
    Position { x: i32, y: i32 },
    ColorType(Color),
    Content(String),
}
fn main() {
    let msg = Msg::Quit;
    match msg {
        // 只能用字面量来匹配没有值的
        Msg::Quit => {
            println!("退出");
        }
        Msg::Position { x, y } => {
            println!("坐标: ({}, {})", x, y);
        }
        Msg::ColorType(Color::Rgb(r, g, b)) => {
            println!("RGB颜色: ({}, {}, {})", r, g, b);
        }
        Msg::ColorType(Color::Hsv(r, g, b)) => {
            println!("Hsv颜色: ({}, {}, {})", r, g, b);
        }
        Msg::Content(txt) => {
            println!("聊天内容: {}", txt);
        }
    }
}
 */

/**
 * 同时解构元组、结构体
 */
/* struct Struct {
    x: i32,
    y: i32,
}
#[allow(unused_variables, dead_code)]
fn main() {
    let ((f, g), Struct { x: h, y: i }) = ((10, 20), Struct { x: 1, y: 2 });
    println!("f: {} g: {} h: {} i: {}", f, g, h, i);
} */

/**
 * 多匹配
 */
/* fn main() {
    let val1 = Some(1);
    let val2 = Some(2);
    match (val1, val2) {
        // 忽略val1的值，忽略va2整个
        (Some(_), _) => {
            println!("ok");
        }
        _ => {
            println!("nothing");
        }
    }
} */

/**
 * 匹配守卫
 */
/* fn main() {
    let num = 1;
    let max = 2;
    match num {
        // 匹配守卫作用于每一项子匹配 => (1|2|3) if ...
        // 匹配守卫允许访问匹配外的变量
        1 | 2 | 3 if num < max => {
            println!("num: {} < max", num); // ✅
                                            // println!("num < max: {max}", max); // ❌
        }
        _ => {
            println!("default");
        }
    }
}
 */

/**
 * 绑定
 */
/* fn main() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // 绑定 id_variable变量的值在[3, 7]
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello {
            // ⚠️指定id变量的范围在[10, 12]，未使用绑定所以无法使用`id`变量
            id: 10..=12,
        } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
} */

/**
 * 1.53+ 绑定语法
 */
/* fn main() {
    let num = 1;
    match num {
        // 将1或者2绑定到res上
        res @ (1 | 2) => {
            println!("1 | 2");
        }
        _ => (),
    }
}
 */

/**
 * 1.56+ if let 使用绑定
 */
/* #[allow(unused_variables, dead_code)]
fn main() {
    #[derive(Debug)]
    struct Point {
        x: String,
        y: String,
    }
    let point = Point {
        x: "1".to_string(),
        y: "2".to_string(),
    };
    if let p @ Point { x: _, y: _ } = point {
        // println!("point x = 1, y = {}", y);
        println!("{:?}", p); // Point { x: 1, y: 2 }
    }
}
 */

/**
 * @绑定整个匹配的值
 */
/* fn main() {
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        // 绑定整个值
        res @ Message::Hello { id: 3..=7 } => {
            println!("Found an id in range: {:?}", res)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
} */

/**
 * 模式匹配可变引用
 */
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        // r的所有权移给了value
        value => {
            value.push_str("rust");
            println!("{}", value);
        }
    } // value销毁
      // println!("{}", r); // ❌ r不再可用
    println!("{}", v); // ✅
}
