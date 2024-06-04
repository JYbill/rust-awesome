/*fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}*/



// 修复代码中的错误
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);
fn main() {
    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };

    pair.cmp_display();
}
