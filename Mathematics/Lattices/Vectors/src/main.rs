use crate::point::{*};

mod point;

fn main() {
    let v = Point {
        x: 2,
        y: 6,
        z: 3,
    };

    let w = Point {
        x: 1,
        y: 0,
        z: 0,
    };

    let u = Point {
        x: 7,
        y: 7,
        z: 2,
    };
    // 3∗(2∗v−w)∙2∗u
    let result = v.mul(2).sub(w).mul(3).dot(u.mul(2));
    
    
    println!("Result: {}", result);
}
