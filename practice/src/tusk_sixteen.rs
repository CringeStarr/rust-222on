#[test]
fn test16(){
    use std::collections::HashMap;
    //Код жахливий хехехе. Я його сам таким зробив, але чат гпт трохи допомагав інколи


    //m, u, x, a, s, l, o, n
    // muxa
    //x   a
    // ------
    // slon

    fn to_numbers(slon: [[char; 8]; 4]) -> [[char; 8]; 4]{
        let mut result: [[char; 8]; 4] = [[' '; 8]; 4];
        let mut counter = 1;
        let mut char_to_digit: HashMap<char, char> = HashMap::new();

        for i in 0..slon.len() {
            for j in 0..slon[i].len() {
                if slon[i][j].is_alphabetic() {
                    let digit = char_to_digit.entry(slon[i][j]).or_insert_with(|| {
                        let digit_char = char::from_digit(counter, 10).unwrap();
                        counter += 1;
                        if counter > 8 {
                            counter = 1;
                        }
                        digit_char
                    });

                    result[i][j] = *digit; // Присваиваем цифру
                } else {
                    result[i][j] = slon[i][j];
                }
            }
        }

        result
    }


    let muxa: [[char; 8]; 4] = [
        [' ', 'm', 'u', 'x', 'a', ' ', ' ', ' '],
        ['x', ' ', ' ', ' ', 'a', ' ', ' ', ' '],
        [' ', '-', '-', '-', '-', '-', '-', '-'],
        [' ', 's', 'l', 'o', 'n', ' ', ' ', ' '],
    ];

    let result = to_numbers(muxa);

    for row in result.iter() {
        // Преобразуем строку в String и выводим
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}