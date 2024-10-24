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
    let h = s1[0]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
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
    assert!(std::mem::size_of_val(&arr) == arr0[arr0.len() - 1]);

    println!("Success!");
}