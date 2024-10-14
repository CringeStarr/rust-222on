#[test]
fn test() {
    let value = 4;

    fn draw(value: i32) {
        for i in 0..value {
            if i < 2 {   //Количество уровней треугольника (в 1 и 2 треугольнике всегда 3 уровня. Потом увеличиваеться на 1 каждый раз
                for j in 0..3 {
                    //первый уровень всегда 1, второй 3, третий 5 и тд
                    print!("{}", "*".repeat(j as usize + 2));
                    println!();
                }
            } else {
                for a in 0..i + 2 {
                    print!("{}", "*".repeat(a as usize + 2));
                    println!();
                }
            }
        }
    }

    draw(value);
}