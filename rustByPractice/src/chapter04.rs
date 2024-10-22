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

    let v1 = 251_i32 + 8;
    let v2 = i32::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

#[test]
fn test6() {
    // Modify `assert!` to make it work
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

#[test]
fn test7() {
    // Fill the blank to make it work

    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

#[test]
fn test8() {
    fn main() {
        assert!(0.1 + 0.2 == 0.4 - 0.1);

        println!("Success!");
    }
}

#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u32);
    }
}

#[test]
fn test10() {
    // Fill the blanks
    use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
fn test11() {

    // Fill the blanks and fix the errors
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

//04.2

#[test]
fn test12() {
    // Make it work
    use std::mem::size_of_val;

    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

#[test]
fn test13() {
    // Make it work

    let c1 = '中';
    print_char(c1);

    fn print_char(c: char) {
        println!("{}", c);
    }
}

#[test]
fn test14() {
    // Make println! work
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test15() {
    // Make it work
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test16() {
    // Make it work, don't modify `implicitly_ret_unit` !
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }
}

#[test]
fn test17() {
    // Modify `4` in assert to make it work
    use std::mem::size_of_val;

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

//04.3

#[test]
fn test18() {
    // Make it work with two ways
    let v = {
        let mut x = 1;
        x += 2;

        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test19() {
    let v = 3;

    assert!(v == 3);

    println!("Success!");
}

#[test]
fn test20() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}

//04.4

#[test]
fn test21() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}

#[test]
fn test22() {
    print();

    // Replace i32 with another type
    fn print() -> () {
        println!("Success!");
    }
}

#[test]
fn test23() {
    // Solve it in two ways
    // DON'T let `println!` work

    never_return();
    println!("Failed!");

    fn never_return() -> ! {
        // Implement this function, don't modify the fn signatures
        panic!();
    }
}

#[test]
fn test24() {
    println!("Success!");

    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        };

        // Rather than returning a None, we use a diverging function instead
        never_return_fn()
    }

    // IMPLEMENT this function in THREE ways
    fn never_return_fn() -> ! {
        panic!();
    }
}

#[test]
fn test25() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}