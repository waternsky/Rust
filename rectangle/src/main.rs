
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}


impl Rectangle {

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn scale(&self, scale_factor: f64) -> Rectangle {
        Rectangle {
            width: self.width * scale_factor,
            height: self.height * scale_factor,
        }
    }
}



fn main() {
    
    let rect1 = Rectangle {
    width: 30.0,
    height: 50.0,
    };
    println!("##### Rectangle 1 #####");
    println!("Rectangle 1: {:#?}", rect1);
    println!("Area of rectangle 1: {}", rect1.area());
    println!("Perimeter of rectangle 1: {}", rect1.perimeter());

    let scale = 2.0;
    println!("Scaling the rectangle by factor {scale}");
    println!("##### Rectangle 2 #####");
    let rect2 = rect1.scale(scale);
    println!("Rectangle 2: {:#?}", rect2);
    println!("Area of rectangle 2: {}", rect2.area());
    println!("Perimeter of rectangle 2: {}", rect2.perimeter());

}
