#[test]
fn test() {
    let data =
        [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];


    for ((a, b), exp) in data.iter() {
        assert_eq!(*exp, gcd(*a, *b));
    }

    fn gcd(a: u32, b: u32) -> u32 {
        //Зробив алгоритмом Евкліда
        let result;
        let a = a;
        let b = b;

        if a == 0{
            result = b;
        }
        else if b == 0{
            result = a;
        }
        else {
            result = gcd(b, a % b);
        }

        result
    }
}
