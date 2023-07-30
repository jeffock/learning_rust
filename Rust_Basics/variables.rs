// VARIABLES
// Assigned using 'let'
// print!() and println!()
// The scope of a variable is defined by the block of code it's declared in
// Shadowing allows a var to be re-declared in the same scope same way

// Binding & Mutability
fn main() {

    let x: i32 = 5;
    let _y: i32; // if uninitialized AND unused, only warning
                 // adding underscore will remove warning

    assert_eq!(x, 5); // asserts that x and 5 are equal
                      // if they aren't equal program will panic

    println!("Success");

    // use mut to mark variable as mutable
    // variables are naturally immutable
    let mut z = 1;
    z += 2;

    assert_eq!(z, 3);
    println!("Success");

    // Scope
    let a: i32 = 10;
    // new scope
    {
        let b: i32 = 5;
        println!("a is {} and b is {}", a, b);
    }
    // var b is not valid here, outside of original scope

}
