fn main() {
    let (star, space) = ('*', ' ');
    let (height, mut stars_value, mut space_value) = (11, 1, 6);

    fn drawing(space_value: i32, stars_value: i32, star: char, space: char) {
        for _ in 0..space_value {
            print!("{}", space);
        }

        for _ in 0..stars_value {
            print!("{}", star);
        }

        println!();
    }

    for i in 0..height {
        drawing(space_value, stars_value, star, space);

        if i <= 4 {
            stars_value += 2;
            space_value -= 1;
        } else {
            stars_value -= 2;
            space_value += 1;
        }
    }
}