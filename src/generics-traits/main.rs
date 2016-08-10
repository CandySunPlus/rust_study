use std::f64::consts;

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

pub fn main() {
    let c = Circle {
        radius: 1.0f64,
    };

    let s = Square {
        side: 1.0f64
    };

    print_area(c);
    print_area(s);
}
