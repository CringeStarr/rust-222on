#[test]
fn test13() {
    use rand::Rng;

    //функція яка рахує мінімальну кількість переносу грузу щоб на всіх кораблях був однаковий груз
    fn count_permutation(shipments: &Vec<u32>) -> usize {
        let sum: u32 = shipments.iter().sum(); //Сума усього вантажа
        let ship_value = shipments.len() as u32; //Кількість кораблів
        let mut changes: u32 = 0;

        if sum % ship_value == 0 {
            let mid = sum / ship_value; //Середнє значення вантажу на кораблях

            for &shipment in shipments {
                if shipment > mid {
                    changes += shipment - mid;
                }
            }

            changes as usize
        } else { 0 }
    }


    //Чи завжди можливо всі кораблі забезпечити однаковою кількість грузу?

    //як буде виглядати сігнатура в іншому випадку?

    fn gen_shipments(n: usize) -> Vec<u32> {
        let mut vec: Vec<u32>;

        loop {
            let mut rng = rand::thread_rng();
            vec = (0..n).map(|_| rng.gen_range(10..100)).collect();

            if vec.iter().sum::<u32>() % vec.len() as u32 == 0 {
                return vec;
            }
        }
    }

    println!("result: {}", count_permutation(&gen_shipments(5)));
}