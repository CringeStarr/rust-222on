#[test]
fn test() {
    const W: u32 = 30;
    const H: u32 = 15;

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1 || y * 2 + x == 28;
            let is_vertical = x == 0 || x == W - 1 || x == 2 * y;

            let c = if is_horizontal || is_vertical{
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}