// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

