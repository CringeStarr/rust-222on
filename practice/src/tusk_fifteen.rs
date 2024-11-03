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
    xs.iter().map(|x| {
        let height = if x.a.y < x.b.y { x.b.y - x.a.y } else { x.a.y - x.b.y };
        let width = if x.a.x < x.b.x { x.b.x - x.a.x } else { x.a.x - x.b.x };
        height * width
    }).sum();

    todo!()
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