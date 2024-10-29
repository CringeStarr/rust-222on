#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate2(s, n),
                exp.to_string()
            )
        );
}


fn rotate2(s: &str, n: &isize) -> String {
    let len = s.len() as isize;
    let shift = (n % len + len) % len; // Нормализация сдвига в пределах длины строки
    let shift = shift as usize;

    format!("{}{}", &s[len as usize - shift..], &s[..len as usize - shift])
}

//Кажу правду - зробив з чатом гпт. Вибачте, я не міг...