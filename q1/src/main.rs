enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(s1) => *s1 * *s1 * std::f64::consts::PI,
            Shape::Rectangle(s1, s2) => *s1 * *s2,
            Shape::Triangle(s1, s2, s3) => {
                let s: f64 = (*s1 + *s2 + *s3) / 2.0;
                (s * (s - *s1) * (s - *s2) * (s - *s3)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(s1) => *s1 * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(s1, s2) => *s1 * 2.0 + *s2 * 2.0,
            Shape::Triangle(s1, s2, s3) => *s1 + *s2 + *s3,
        }
    }

    fn twoperimeter(&self) -> f64 {
        match self {
            Shape::Circle(s1) => *s1 * 2.0 * std::f64::consts::PI * 2.0,
            Shape::Rectangle(s1, s2) => (*s1 * 2.0 + *s2 * 2.0) * 2.0,
            Shape::Triangle(s1, s2, s3) => (*s1 + *s2 + *s3) * 2.0,
        }
    }

    fn validity(&self) -> bool {
        match self {
            Shape::Circle(s1) => *s1 >= 0.0,
            Shape::Rectangle(s1, s2) => *s1 >= 0.0 && *s2 >= 0.0,
            Shape::Triangle(s1, s2, s3) => {
                *s1 >= 0.0 && *s2 >= 0.0 && *s3 >= 0.0 &&
                *s1 + *s2 > *s3 &&
                *s2 + *s3 > *s1 &&
                *s1 + *s3 > *s2
            }
        }
    }    
}

fn main() {
    let example = Shape::Circle(45.0);
    println!("{}", example.area());
    println!("{}", example.perimeter());
    println!("{}", example.twoperimeter());
    println!("{}", example.validity());
}