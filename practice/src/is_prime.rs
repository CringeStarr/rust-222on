#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data
        .iter()
        .for_each(|(n, prime)|
            assert_eq!(is_prime(n), *prime)
        )
}

fn is_prime(n: &u32) -> bool {
    let mut factorial = 0;
    let result: bool;

    if *n < 2 { result = false } else {
        for i in 1..*n {
            factorial += i;
        }

        result = (factorial + 1) % n == 0;
    }

    result
}