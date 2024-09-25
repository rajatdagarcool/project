enum Message{
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8,u8),
}

fn process_message(msg: Message){
    match msg{
        Message::Quit => println!("Quit message Received"),
        Message::Move{x, y} => println!("Move to position: ({} {})", x,y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r,g,b) => println!("Change color to RGB ({}, {},{})", r,g,b),
    }
}

fn main(){
    let msg1 = Message::Move{ x: 10, y: 20};
    let msg2 = Message::Write(String::from("Hello, world!"));

    process_message(msg1);
    process_message(msg2);
}
