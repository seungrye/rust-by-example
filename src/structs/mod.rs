pub fn main() {
    let point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // point 구조체로부터 멤버 변수의 값을 my_x, my_y 로 획득함.
    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        p1: point,
        p2: Point {
            x: my_x + 2.0,
            y: my_y + 2.0,
        },
    };
    println!("rectangle area: {}", _rectangle.rect_area());

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("square : {:?}", square(&_rectangle.p2, 10.0));
}
/// 유닛 구조체
struct Nil;

/// 튜플 구조체
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn rect_area(&self) -> f32 {
        let Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        } = *self;

        return (x2 - x1) * (y2 - y1);
    }
}

fn square(point: &Point, v: f32) -> Rectangle {
    return Rectangle {
        p1: Point {
            x: point.x,
            y: point.y,
        },
        p2: Point {
            x: point.x + v,
            y: point.y + v,
        },
    };
}
