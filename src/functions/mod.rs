// pub mod methods;
// pub mod closures;
// pub mod higher_order_functions;

pub fn main() {
    methods();
    closures();
}
fn closures() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 인자를 취하지 않는 closure 이며, 1을 반환한다.
    // 반환 타입은 추정됨.
    let one = || 1;
    println!("closure returning one: {}", one());
}
fn methods() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1., 1.),
    };

    square.translate(1., 1.);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        return ((x1 - x2) * (y1 - y2)).abs();
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        return 2. * ((x1 - x2).abs() * (y1 - y2).abs());
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn origin() -> Point {
        return Point { x: 0., y: 0. };
    }

    fn new(x: f64, y: f64) -> Point {
        return Point { x: x, y: y };
    }
}
