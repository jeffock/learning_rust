// BROKE

use std::io;

fn main() {
    let mut n = 0u16;

    while n == 0 {
        *&mut n = gloop()
    }
}

fn gloop() -> u16{
    println!("name?");
    loop {
        let input = io::Stdin::read_line();
        match input.expect("failed to read line").as_ref() {
            "jeffy" => return 1,
            _ => return 0,
        }
    }
}
