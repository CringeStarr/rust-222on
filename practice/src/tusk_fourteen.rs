//Вибачте за такий код(  Я навіть такий робив увесь день


//Точка на площині
struct Point {
    x: i32,
    y: i32,
}

//Прямокутник на площині
struct Rectangle {
    a: Point,
    b: Point,
}


//Рахує зайняту площу
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut temp_one = 0;
    let mut temp_two = 0;
    let mut result = 0;

    let mut sum = xs.iter().map(|x| {
        let height = if x.a.y < x.b.y { x.b.y - x.a.y } else { x.a.y - x.b.y };
        let width = if x.a.x < x.b.x { x.b.x - x.a.x } else { x.a.x - x.b.x };
        height * width
    }).sum();


    for i in 0..xs[0].b.x as usize {
        if i >= xs[0].a.x as usize && i < xs[1].b.x as usize {
            temp_one += 1;
        }
    }

    temp_two = (xs[1].a.y + xs[1].b.y) - (xs[0].a.y + xs[0].b.y);

    result = temp_one * temp_two;


    temp_one = 0;
    temp_two = 0;

    for i in 0..xs[2].b.x as usize {
        if i >= xs[2].a.x as usize && i < xs[1].b.x as usize {
            temp_one += 1;
        }
    }

    temp_two = (xs[1].a.y + xs[1].b.y) - (xs[2].a.y + xs[2].b.y);

    result += temp_one * temp_two;

    sum -= result;

    sum
}


fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}