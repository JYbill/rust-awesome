/*
  枚举值
*/
/* #[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart); // Hearts
    print_suit(diamond); // Diamonds
}
fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
} */

/*
  枚举 + 值
*/
/* #[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);
    println!("{:?}", c1);
} */

/*
  Option 枚举处理空值
*/
#[allow(unused_variables)]
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five {:?}", none);
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
