/* 1. _变量，忽略未使用变量 */
/* fn main() {
    let _x = 11; // ✅
                 // let y = 12; //  ⚠️warning：unused variable `y`
    print!("hello")
} */

/*
// 2. 变量结构
fn main() {
    let (foo1, mut foo2): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", foo1, foo2);

    foo2 = false;
    assert_eq!(foo1, foo2); // 断言操作，如果不相等即抛出错误
    print!("hello") // 只有断言成功才能继续向下执行
}
 */

// 结构赋值使用其他类型
/* struct Struct {
    e: i32,
    _num2: i32,
}
fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2); // 元组结构, a = 1, b = 2
                     // _ 表示忽略掉结构的值，因为我们不关心它是什么，也不需要它
    [c, .., _, d] = [1, 2, 3, 4, 5]; // 数组结构, c = 1, d = 5
    Struct { e, .. } = Struct { e: 5, _num2: 10 }; // e = 5
    assert_eq!([1, 2, 1, 5, 5], [a, b, c, d, e]); // ✅
    println!("ok.")
} */

// 变量覆盖
fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
