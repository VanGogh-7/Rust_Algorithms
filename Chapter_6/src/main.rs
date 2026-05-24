mod ip_address;
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
enum Option<T> {
    None,
    Some(T),
}
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 0);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let some_number = Some(5);

    let absent_number: Option<i32> = None;
}
