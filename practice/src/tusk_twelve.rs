#[test]
fn test() {
    use rand::Rng;

    fn gen_random_vector(n: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..n).map(|_| rng.gen_range(10..100)).collect()
    }

    fn min_adjacent_sum(data: &[i32]) -> (i32, i32, i32, i32, i32) {
        let mut result = i32::MAX;
        let mut numbers = (0, 0);
        let mut indexes = (0, 0);

        for i in 0..data.len() - 1 {
            if data[i] + data[i + 1] < result {
                result = data[i] + data[i + 1];
                numbers = (data[i], data[i + 1]);

                indexes.0 = i;
                indexes.1 = i + 1;
            }
        }

        return (numbers.0, numbers.1, indexes.0 as i32, indexes.1 as i32, result);
    }

    let data: Vec<i32> = gen_random_vector(20);
    let result = min_adjacent_sum(&data[..]);
    // min adjacent sum=21+12=33 at indexes:11,12

    println!("min adjacent sum={}+{}={} at indexes:{:?},{:?}", result.0, result.1, result.4, result.2, result.3);
}