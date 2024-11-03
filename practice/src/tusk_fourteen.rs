struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let sum: i32 = xs.iter().map(|x| {
        let height = if x.a.y < x.b.y { x.b.y - x.a.y } else { x.a.y - x.b.y };
        let width = if x.a.x < x.b.x { x.b.x - x.a.x } else { x.a.x - x.b.x };
        height * width
    }).sum();

    fn delete_common_area(rec_one: &Rectangle, rec_two: &Rectangle) -> i32 {
        let mut common_width = 0;
        let mut common_height = 0;
        let result;

        for i in 0..rec_one.b.x as usize {
            if i >= rec_one.a.x as usize && i < rec_two.b.x as usize {
                common_width += 1;
            }
        }

        for i in 0..rec_one.a.y as usize {
            if i >= rec_one.b.y as usize && i >= rec_two.b.y as usize && i < rec_two.a.y as usize {
                common_height += 1;
            }
        }

        result = common_width * common_height;

        result
    }

    let overlap_area_1_2 = delete_common_area(&xs[0], &xs[1]);
    let overlap_area_1_3 = delete_common_area(&xs[2], &xs[1]);

    sum - (overlap_area_1_2 + overlap_area_1_3)
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