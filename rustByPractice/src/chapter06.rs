//06.1

#[test]
fn test() {
    // Fix error without adding new line
    let s = "hello, world";

    println!("Success!");
}

#[test]
fn test2() {

    // Fix the error with at least two solutions
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}

#[test]
fn test3() {
    // Fill the blank
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]
fn test4() {
    // Fix all errors without adding newline
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test5() {
    // Fill the blank
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6() {
    // Fix errors without removing any line
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

#[test]
fn test7() {
    // Fix error with at least two solutions
    let s = "hello, world";
    greetings(s.to_string());


    fn greetings(s: String) {
        println!("{}", s)
    }
}

#[test]
fn test8() {
    // Use two approaches to fix the error and without adding a new line
    let s = "hello, world".to_string();
    let s1 = s;

    println!("Success!");
}

#[test]
fn test9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name);

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10() {
    /* Fill in the blank and fix the errors */
    // Используем обычную строку, чтобы escape-последовательности работали
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // Если нужны кавычки в сырой строке, добавляем # в делимитеры
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Если нужно использовать # в строке, добавляем больше # в делимитеры
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Строка с длинным делимитером
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
    //Вибачте, зробив з чатом гпт це, тому що не знав як
}

#[test]
fn test11() {
    let s1 = String::from("hi,中国");

    let h = s1.chars().nth(0).unwrap(); // Получаем первый символ
    assert_eq!(h, 'h'); // `h` является символом, поэтому используем одинарные кавычки

    let h1 = &s1[3..6]; // Используем диапазон байтов для "中"
    assert_eq!(h1, "中");

    println!("Success!");
    //не знаю як це зробити
}

#[test]
fn test12() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

#[test]
fn test13() {
    use utf8_slice;

    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // Will equal "🚀"

    //Теж не знаю(
}

//06.2

#[test]
fn test14() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

#[test]
fn test15() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert_eq!(std::mem::size_of_val(&arr), arr0[arr0.len() - 1]);

    println!("Success!");
}

#[test]
fn test16() {
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

#[test]
fn test17() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}

#[test]
fn test18() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}

#[test]
fn test19() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names.get(1).unwrap();

    println!("Success!");
}

//06.3
#[test]
fn test20() {
    // Fix the errors, DON'T add new lines!

    let arr = [1, 2, 3];
    let s1 = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test21() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

#[test]
fn test22() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

#[test]
fn test23() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

#[test]
fn test24() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}

#[test]
fn test25() {
    // Fix errors

    let s = &mut String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter(&s);


    println!("the first letter is: {}", letter);
    s.clear(); // error!

    fn first_letter(s: &str) -> &str {
        &s[..1]
    }
}

//06.4

#[test]
fn test26() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

#[test]
fn test27() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}

#[test]
fn test28() {
    let too_long_tuple = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn test29() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

#[test]
fn test30() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

#[test]
fn test31() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");


    fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }
}

//06.5

#[test]
fn test32() {
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }

    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "sd".to_string(),
    };

    println!("Success!");
}

#[test]
fn test33() {
    struct Unit;
    trait SomeTrait {
        // ...Some behaviors defined here.
    }

    // We don't care about what fields  are  in the Unit, but we care about its behaviors.
    // So we use a struct with no fields and implement some behaviors for it
    impl SomeTrait for Unit {}

    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");

    // Fill the blank to make the code work
    fn do_something_with_unit(u: Unit) {}
}

#[test]
fn test34() {
    // Fix the error and fill the blanks
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");


    fn check_color(p: Point) {
        let x = p.0;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(p.2, 255);
    }
}

#[test]
fn test35() {
    // Fill the blank and fix the error without adding/removing new line
    struct Person {
        name: String,
        age: u8,
    }

    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

#[test]
fn test36() {
    // Fill the blank
    struct Person {
        name: String,
        age: u8,
    }

    println!("Success!");

    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name,
        }
    }
}

#[test]
fn test37() {
    // Fill the blank to make the code work
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

#[test]
fn test38() {
    // Fill the blanks to make the code work
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

#[test]
fn test39() {
    // Fix errors to make it work
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}, {:?}", _name, f.data, "File");
}


//06.6
#[test]
fn test40() {
    // Fix the errors
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C-like enum
    enum Number2 {
        Zero = 0.0 as isize,
        One = 1.0 as isize,
        Two = 2.0 as isize,
    }

    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as isize, Number1::One as isize);
    assert_eq!(Number1::One as isize, Number2::One as isize);

    println!("Success!");
}

#[test]
fn test41() {
    // Fill in the blank
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Move { x: 1, y: 2 }; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write(String::from("hello, world")); // Instantiating with "hello, world!"

    println!("Success!");
}

#[test]
fn test42() {
    // Fill in the blank and fix the error
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move { x: 2, y: 2 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

#[test]
fn test43() {
    // Fill in the blank and fix the errors
    #[derive(Debug)]

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }
}

#[test]
fn test44() {
    // Fill in the blank to make the `println` work.
    // Also add some code to prevent the `panic` from running.

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    }

    //panic!("NEVER LET THIS RUN！");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}

#[test]
fn test45() {
    // use crate::List::*;
    //
    // enum List {
    //     // Cons: Tuple struct that wraps an element and a pointer to the next node
    //     Cons(u32, Box<List>),
    //     // Nil: A node that signifies the end of the linked list
    //     Nil,
    // }
    //
    // // Methods can be attached to an enum
    // impl List {
    //     // Create an empty list
    //     fn new() -> List {
    //         // `Nil` has type `List`
    //         List::Nil
    //     }
    //
    //     // Consume a list, and return the same list with a new element at its front
    //     fn prepend(self, elem: u32) -> List {
    //         // `Cons` also has type List
    //         Cons(elem, Box::new(self))
    //     }
    //
    //     // Return the length of the list
    //     fn len(&self) -> u32 {
    //         // `self` has to be matched, because the behavior of this method
    //         // depends on the variant of `self`
    //         // `self` has type `&List`, and `*self` has type `List`, matching on a
    //         // concrete type `T` is preferred over a match on a reference `&T`
    //         // After Rust 2018 you can use self here and tail (with no ref) below as well,
    //         // rust will infer &s and ref tail.
    //         // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    //         match *self {
    //             // Can't take ownership of the tail, because `self` is borrowed;
    //             // Instead take a reference to the tail
    //             Cons(_, ref tail) => 1 + tail.len(),
    //             // Base Case: An empty list has zero length
    //             Nil => 0
    //         }
    //     }
    //
    //     // Return representation of the list as a (heap allocated) string
    //     fn stringify(&self) -> String {
    //         match *self {
    //             Cons(head, ref tail) => {
    //                 // `format!` is similar to `print!`, but returns a heap
    //                 // allocated string instead of printing to the console
    //                 format!("{}, {}", head, tail.stringify())
    //             }
    //             Nil => {
    //                 format!("Nil")
    //             }
    //         }
    //     }
    // }
    //
    //
    // // Create an empty linked list
    // let mut list = List::new();
    //
    // // Prepend some elements
    // list = list.prepend(1);
    // list = list.prepend(2);
    // list = list.prepend(3);
    //
    // // Show the final state of the list
    // println!("linked list has length: {}", list.len());
    // println!("{}", list.stringify());
}