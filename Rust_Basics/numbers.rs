// Lengths: 8, 16, 32, 64, 128 bit and arch
// Signed is positive and negative, unsigned is only positive
// i8, i16, etc. for signed and u8, u16, etc. for unsigned
// isize and usize for arch length - dependent of pc architecture

// Default types: i32 and f64

// usize/isize guarantees that it will be big enough to hold pointer

// f32 & f64

#[allow(unused)]

fn main() {

    let x: i32 = 5;
    let mut _y: u32 = 5;

    //y = x; this would fail, different types
    
    let mut y: i32 = 5;
    y += 1;

    y = x;



    let v: u16 = 38_u8 as u16; // use as to change type
    


    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));



    assert_eq!(i8::MAX, 127); // _::MAX returns biggest possible value
    assert_eq!(u8::MAX, 255);



    // let v1 = 251_u8 + 8; // var becomes u8 and 8_u8
                         // issue: 251 + 8 > 255
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();



    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // decimal, hex, oct, binary
                                             // 1024 + 255, 63, 255
                                             // = 1597
    assert!(v == 1597);
}

// get the type of given var and return string rep of type
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

