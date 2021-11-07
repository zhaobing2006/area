use std::pin;

fn main() {

    let triangle = Triangle{
        side:50,
        altitude:60,
    };
    println!("The area of this triangle is {}",triangle.area());

    let rectangle = Rectangle{
        long: 30,
        width: 40,
    };
    println!("The area of this rectangle is {}",rectangle.area());

    compute_area(triangle);
    compute_area(rectangle);
}

pub trait Area {
    fn area(&self)->u32;
}


struct Triangle{
    side: u32,
    altitude: u32,
} 

impl Area for Triangle {
    fn area(&self) -> u32{
        let area=  (self.side * self.altitude)/2;
        area
    }
}

struct Rectangle{
    long: u32,
    width: u32,
}

impl Area for Rectangle{
    fn area(&self) -> u32{
        let area =self.long * self.width;
        area
    }
}

fn compute_area<T:Area>(any:T){
    println!("we can compute area of triangle and rectangle");
    let area= any.area();
    println!("the area of this tangle is {}",area);
}