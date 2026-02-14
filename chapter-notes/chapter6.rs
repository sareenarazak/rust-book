// enums --> used to represent data when there is a finite variants of something
// each variant can have different types and amounts of data associated with it 

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn distance(&self) -> u32 {
        return 20;
    }
}

fn main() {
    let message = Message::Move{x:2, y: 1};
    println!("distance is : {0}",message.distance());
    
    // option enum has two variants Some and None
    let some_value : Option<i32> = Some(10);
    let amount = match some_value {
        Some(value) => value,
        _ =>    0,
    };

    println!("amount is {amount}");
    let no_value : Option<i32> = None;
    
    let amount = match no_value {
        Some(value) => value,
        _ =>    0,
    };
    println!("amount is {amount}");

    //if let syntax can be used to write concise control flow 
    let write = Message::Write(String::from("inji says woof"));
    if let  Message::Write(message) =  write {
        println!("message is {message}");
    }

    // similarly let else syntax can be used for else being happy path
}
