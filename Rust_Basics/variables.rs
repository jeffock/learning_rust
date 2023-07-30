// VARIABLES
// Assigned using 'let'
// print!() and println!()
// The scope of a variable is defined by the block of code it's declared in
// Shadowing allows a var to be re-declared in the same scope same way

// Binding & Mutability
fn main() {

    let x: i32 = 5;

    assert_eq!(x, 5); // asserts that x and 5 are equal
                      // if they aren't equal program will panic 
    println!("Success");

}
