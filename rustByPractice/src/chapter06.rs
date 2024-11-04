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