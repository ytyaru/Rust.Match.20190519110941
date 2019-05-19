use std::cmp::Ordering;
fn main() {
    let secret_number = 0;
    let guess = 0; // -1, 1
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小さすぎます。残念……"),
        Ordering::Greater => println!("大きすぎます。残念……"),
        Ordering::Equal => println!("ぴったり！　やったね！"),
    }
}
