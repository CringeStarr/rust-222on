fn main() {
    let (star, space) = ('*', ' ');
    let (mut stars_value, mut space_value) = (1, 6);
    const HEIGHT: i32 = 11;

    for i in 0..HEIGHT {
        print!("{}{}", space.to_string().repeat(space_value as usize), star.to_string().repeat(stars_value as usize));
        println!();

        if i <= 4 {
            stars_value += 2;
            space_value -= 1;
        } else {
            stars_value -= 2;
            space_value += 1;
        }
    }
}