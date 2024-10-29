#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}

fn is_palindrome(x: u32) -> bool {
    let x_to_str = x.to_string();
    let x_len = x_to_str.len();
    let mut reverse = String::new();

    for i in (0..x_len).rev() {
        reverse.push(x_to_str.as_bytes()[i] as char);
    }

    return x_to_str == reverse;
}
