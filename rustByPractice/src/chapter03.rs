#[test]
fn test1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test2() {
    // Fill the blanks in the code to make it compile
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {

    // Fix the error below with least amount of modification

    let x: i32 = 10;
    let y: i32;
    {
        y = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn test4() {
    // Fix the error with the use of define_x

    let x = define_x();
    println!("{}, world", x);


    fn define_x() -> &'static str {
        let x = "hello";
        x
    }
}

#[test]
fn test5() {
    // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".

}

#[test]
fn test6() {
    // Remove a line in the code to make it compile

    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let _x = x;


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn test7() {
    let x = 1;
    println!("{}", x);


    // Warning: unused variable: x
}

#[test]
fn test8() {
    // Fix the error below with least amount of modification

    let (x, y) = (1, 2);
    let x = x + 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn test9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}