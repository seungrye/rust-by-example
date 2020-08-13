pub fn main() {
    methods();
    closures();
    capturing();
    closure_as_input_parameters();
}
// 가능한 closure 타입은 다음과 같다.
// Fn : 클로저의 캡처 변수를 참조로 획득 (&T)
// FnMut : 클로저의 캡처 변수를 가변 참조로 획득(&mut T)
// FnOnce : 클로저의 캡처 변수를 값을 획득(T)
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
// i32 를 반환하는 클로저를 파라메터로 받는 함수이며, 이 함수는 i32 를 반환한다.
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(3);
}
fn closure_as_input_parameters() {
    let mut farewell = "goodbye".to_owned();
    let greeting = "hello";

    // 2개 변수를 캡처하게 된다. `greeting` 은 참조, `farewell` 은 값
    let diary = || {
        println!("I said {}.", greeting);

        // 변경은 `farewell` 을 가변참조로 캡처되게 강제한다.
        // 다음을 실행하려면 `FnMut` 선언이 필요함
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep ....zzzz");

        // 수동으로 drop 을 호출하면, `farewell` 을 값으로 캡처되도록 강제한다.
        // 다음을 실행하려면 `FnOnce` 선언이 필요함.
        std::mem::drop(farewell);
    };

    apply(diary);

    let double = |x| {
        return 2 * x;
    };
    println!("3 doubled : {}", apply_to_3(double));
}

fn capturing() {
    let color = "green";
    // `color` 를 출력하는 클로저는 `color` 를 즉각적으로 대여(&) 하고
    // print 변수에 borrow와 클로저를 저장합니다.
    // print 가 범위에서 벗어날때까지 borrow 는 유지됩니다.
    let print = || println!("`color`:{}", color);
    print();
    print();

    let mut count = 0;

    // `count` 를 증가시키는 클로저는 `&mut count` 또는 `count` 중 하나를 취할수 있지만,
    // `&mut count` 이 덜 제한적이므로, 이를 취합니다.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    let _reborrow = &mut count;

    let movable = Box::new(3);
    // `mem::drop` 는 `T` 를 요구하므로, 이는 값을 취합니다.
    // 복사 타입은 클로저로 복사되어 원본은 변경되지 않습니다.
    // 복사가 아니라면, 이동되어야 합니다. 따라서, `movable` 이 클로저로 이동됩니다.
    let consume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };
    consume();
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
    println!("closure_inferred: {}", closure_inferred(i)); // note. infer 는 `추론하다` 라는 의미임.

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
