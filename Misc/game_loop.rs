use std::io;

fn main() {
    // user input
    let mut looper = true;
    let mut name = "name";

    while looper {
        println!("What is your name?");

        std::io::stdin().read_line(&mut name.to_owned()).expect("failed to read line");
        
        if &name == "jeffy" {
            *&mut looper = false;
        } else {
            println!("wrong name...");
        }
    }
}
