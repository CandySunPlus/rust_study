// use std::f64::consts;
use std::ops::Mul;

trait HasArea<T> {
    fn area(&self) -> T;
}

// struct Circle<T> {
//     radius: T,
// }
//
// impl<T> HasArea<T> for Circle<T> 
// where T: Mul<Output=T> + Copy {
//     fn area(&self) -> T {
//         consts::PI * (self.radius * self.radius)
//     }
// }

struct Square<T> {
    side: T,
}

impl<T> HasArea<T> for Square<T>
where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

// fn print_area<T: HasArea>(shape: T) {
//     println!("This shape has an area of {}", shape.area());
// }

pub fn main() {
    // let c = Circle {
    //     radius: 1.0f64,
    // };

    let s = Square {
        side: 1.0f64
    };

    // println!("This shape has an area of {}", c.area());
    println!("This shape has an area of {}", s.area());
}
