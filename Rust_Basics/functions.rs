fn main() {
    let (x,y) = (1,2);
    let s: i32 = sum(x,y);

    assert_eq!(s,3);
    println!("Success.");

    print();

    //never_return();
}

fn sum(x: i32, y: i32) -> i32 {
    x+y
}

fn print() -> () {
    println!("Success!");
}

fn get_option(tp:u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return()
}

// diverging function, should never return to caller
fn never_return() -> ! {
    // exit immediately
    unimplemented!()
    //todo!()
    //panic!()
}
