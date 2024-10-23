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

}