struct Polygon {
    side_length: u32,
    num_sides: u32,
}

impl Polygon {
    fn new(side_length: u32, num_sides: u32) -> Self {
        Polygon {
            side_length,
            num_sides,
        }
    }

    fn perimeter(&self) -> u32 {
        self.side_length * self.num_sides
    }

    fn area(&self) -> f64 {
        let apothem: f64 = self.side_length as f64/ (2.0*(std::f64::consts::PI/self.num_sides as f64).tan());
        0.5 * apothem * self.side_length as f64 * self.num_sides as f64
    }

    fn radius(&self) -> f64 {
        let apothem: f64 = self.side_length as f64/ (2.0*(std::f64::consts::PI/self.num_sides as f64).tan());
        return (apothem*apothem + ((self.side_length as f64 * self.side_length as f64)/4.0)).sqrt()
    }

    fn percent_of_circle(&self) -> f64 {
        (self.area() / (std::f64::consts::PI * self.radius() * self.radius())) * 100.0
    }
}

fn main() {
    let numsides = vec![6.0, 12.0, 24.0, 128.0, 256.0, 512.0, 1024.0, 2048.0, 65536.0];
    let sidelength = vec![1.0,2.0,3.0];
    
    for n in &numsides{
        for l in &sidelength{
            let mut polygon = Polygon{side_length: *l as u32, num_sides: *n as u32};
            println!("For a polygon with {} sides of length {}, the area is {} percent of its corresponding circle.", n,l,polygon.percent_of_circle());
        }
    }
}