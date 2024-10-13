// chapter 04.1
#[test]
fn test1() {
    // Remove something to make it work

    let x: i32 = 5;
    let y;

    y = x;

    let _z = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn test2() {
    // Fill the blank

    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test3() {
    // Modify `assert_eq!` to make it work

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");


    //Get the type of given variable, return a string representation of the type, e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

#[test]
fn test4() {
    // Fill the blanks to make it work

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
fn test5() {
    // Fix errors and panics to make it work

    let v1 = 251_u8 + 8;
    let v2 = i8::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

