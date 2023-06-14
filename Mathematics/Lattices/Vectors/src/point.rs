#[derive(Clone, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

pub trait Addition {
    fn add(&self, _: Self) -> Self;
}

impl Addition for Point {
    fn add(&self, point: Point) -> Point {
        Point{ 
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }
}


pub trait Substraction {
    fn sub(&self, _: Self) -> Self;
}

impl Substraction for Point {
    fn sub(&self, point: Point) -> Point {
        Point{ 
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
        }
    }
}


pub trait Multiply {
    fn mul(&self, _: u64) -> Self;
}

impl Multiply for Point {
    fn mul(&self, q: u64) -> Point {
        Point{ 
            x: self.x * q,
            y: self.y * q,
            z: self.z * q,
        }
    }
}



pub trait InnerProduct {
    fn dot(&self,  _: Self) -> u64;
}

impl InnerProduct for Point {
    fn dot(&self, point: Point) -> u64 {
        self.x * point.x + self.y * point.y + self.z * point.z
    }
}
