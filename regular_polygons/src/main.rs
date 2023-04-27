
/* Implementing regular polygons
 * centres are unit distance apart from corners */

const PI: f32 = std::f32::consts::PI;


#[derive(Debug)]
struct Ngon{sides: usize}

impl Ngon {
    fn _number_of_sides(&self) -> f32 {
        self.sides as f32
    }
    fn side_length(&self) -> f32 {
        2.0 * (PI / self._number_of_sides()).sin()            
    }
    fn area(&self) -> f32 {
        (self._number_of_sides() / 2.0) * (2.0 * PI / self._number_of_sides()).sin()
    }
    fn perimeter(&self) -> f32 {
        self._number_of_sides() * self.side_length()
    }
}


fn main() {

    const ITER: usize = 1000;

    let mut area: [f32; ITER] = [0.0; ITER];

    let mut perimeter: [f32; ITER] = [0.0; ITER];

    for i in 3..ITER {
        let polygon = Ngon{sides: i};
        area[i] = polygon.area();    
    }
    
    for i in 3..ITER {
        let polygon = Ngon{sides: i};
        perimeter[i] = polygon.perimeter();    
    }

    println!("Array of areas {:?}", area);
    println!("Array of perimeters {:?}", perimeter);

    println!("Observation Area -> π, Perimeter -> 2π");

    println!("Area goes slowly than perimeter to limit");
}
