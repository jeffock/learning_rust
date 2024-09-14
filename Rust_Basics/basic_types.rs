use std::mem::size_of_val;
fn main() {
    let c1 = 'a'; // 4 bytes
    print_char(c1);
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中'; // "" for str '' for char
    print_char(c2);
    assert_eq!(size_of_val(&c2),4);

    // unit types
    

} 

fn print_char(c : char) {
    println!("{}", c);
}
