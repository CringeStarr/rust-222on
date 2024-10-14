#[test]
fn test() {
    let quantity = 5;

    fn draw(quantity: i32) {
        for i in 0..quantity {
            let mut stars = 3;
            let mut spases = 5;

            println!("{}{}", " ".repeat(spases as usize), "*");   //Перша зірочка завжди одна

            if i == 0 {  //Перший трикутник
                println!("{}{}", " ".repeat(spases as usize), "*");
                println!("{}{}", " ".repeat((spases - 1) as usize), "***");
            } else {   //Всі наступні трикутники
                for _ in 0..(i + 1) {
                    println!("{}{}", " ".repeat((spases - 1) as usize), "*".repeat(stars as usize));
                    stars += 2;
                    spases -= 1;
                }
            }
        }
    }

    draw(quantity);
}