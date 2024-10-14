#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });

    fn invert_the_case(s: String) -> String {
        let result = s.chars().map(|x| {
            if x.is_uppercase() {
                x.to_lowercase().to_string()
            } else if x.is_lowercase() {
                x.to_uppercase().to_string()
            } else { x.to_string() }
        }).collect();

        result
    }
}