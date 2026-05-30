
fn largest0(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest1<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point0<T> {
    x: T,
    y: T,
}
impl<T> Point0<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point1<X2, Y2>,
    ) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 100, 110];

    let result = largest0(&number_list);
    println!("{result}");

    let result = largest1(&number_list);
    println!("{result}");

    let p = Point0 {x:5, y:10};
    println!("p.y = {}",p.y());

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
