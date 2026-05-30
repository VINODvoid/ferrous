trait Area {
    fn area(&self) -> f64;

    fn actual_area(&self) -> String {
        format!(" Actual Area is : {}", self.area())
    }
}

struct Circle {
    radius:f64
}
struct Rectangle{
    height:f64,
    width:f64,
}
impl Area for Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main(){
    let circle = Circle{radius:12.5};
    let rectangle = Rectangle{width:6.8,height:9.4};
    println!("{}",rectangle.actual_area());
    println!("{}",circle.actual_area());
}
