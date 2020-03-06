// simple struct and match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// matching on Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}



fn main() {
    // coin matching
    let pen = Coin::Penny;
    let nick = Coin::Nickel;
    let dime = Coin::Dime;
    let quart = Coin::Quarter;
    println!("The value of a penny is {} cent(s)", value_in_cents(&pen));
    println!("The value of a nickel is {} cent(s)", value_in_cents(&nick));
    println!("The value of a dime is {} cent(s)", value_in_cents(&dime));
    println!("The value of a quarter is {} cent(s)", value_in_cents(&quart));

    // Option<T> matching
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("5 +1 is {:?}, nothing plus one is {:?}", six, none);

    // if let match
    if let Coin::Quarter = quart {
        println!("I was able to match a quarter using if/let");
    }

}
