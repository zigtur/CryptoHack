use std::fmt;

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    field: i64,
    curve: EllipticCurve,
}

#[derive(Copy, Clone, PartialEq)]
struct EllipticCurve {
    a: i64,
    b: i64,
    o: i64,
}

pub trait Operations {
    fn add(&self, secondPoint: &Self) -> Self;
    fn mul(&self, n: usize) -> Self;
}
pub trait Verification {
    fn verificate_elliptic_curve(&self);
}

impl fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {}, {})", self.a, self.b, self.o)
    }
}


impl Operations for Point {

    fn add(&self, secondPoint: &Self) -> Self {
        assert!(self.curve == secondPoint.curve);
        assert!(self.field == secondPoint.field);
        if self.x == 0 && self.y == 0 {
            return Self {
                x: secondPoint.x,
                y: secondPoint.y,
                field: secondPoint.field,
                curve: secondPoint.curve,
            };
        }
        else if secondPoint.x == 0 && secondPoint.y == 0 {
            return Self {
                x: self.x,
                y: self.y,
                field: self.field,
                curve: self.curve,
            };
            
        }
        else {
            if self.x == secondPoint.x && self.y == (0 - secondPoint.y) % secondPoint.field {
                return Self {
                    x: self.curve.o,
                    y: self.curve.o,
                    field: self.field,
                    curve: self.curve,
                };
            }
            else {
                let mut gamma: i64 = 0;
                if self.x != secondPoint.x || self.y != secondPoint.y {
                    let y_calculation = (secondPoint.y - self.y) % self.field;
                    let x_calculation = (secondPoint.x - self.x) % self.field;
                    gamma =  (y_calculation * mod_inv(x_calculation, self.field)) % self.field;
                }
                else {// if (self.x == secondPoint.x && self.y == secondPoint.y) {
                    gamma =  ((3 * self.x.pow(2) + self.curve.a) * mod_inv(2*self.y, self.field)) % self.field;
                }
                let x_3 = (gamma.pow(2) - self.x - secondPoint.x).rem_euclid(self.field); // will make a positive modulo
                let y_3 = (gamma * (self.x - x_3) - self.y).rem_euclid(self.field); // will make a positive modulo
                return Self{
                    x: x_3,
                    y: y_3,
                    field: self.field,
                    curve: self.curve,
                }
            }
        }
    }

    fn mul(&self, n: usize) -> Self {
        let mut q = self.clone();
        let mut r = Point {
            x: 0,
            y: 0,
            field: self.field,
            curve: self.curve,
        };

        let mut num = n;
        while num > 0 {
            if num % 2 == 1 {
                // Set R = R + Q and not the inverse !!!
                r = r.add(&q);
            }
            q = q.add(&q);
            num = num / 2;
        }
        r
    }
}

impl Verification for Point {
    fn verificate_elliptic_curve(&self) {
        let left_member = (self.y.pow(2)) % self.field;
        let right_member = (self.x.pow(3) + self.curve.a * self.x + self.curve.b) % self.field;
        assert!(right_member == left_member);        
    }
}

fn mod_inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
    
    while mn.1 != 0 {
      xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
      mn = (mn.1, mn.0 % mn.1);
    }
    
    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0
  }

fn main() {
    let ellipticCurve = EllipticCurve {
        a: 497,
        b: 1768,
        o: 0,
    };
    let pointX = Point {
        x: 5323,
        y: 5438,
        field: 9739,
        curve: ellipticCurve,
    };

    let pointResult = pointX.mul(1337);
    //pointResult.verificate_elliptic_curve();
    println!("({},{})", pointResult.x, pointResult.y);

    // Challenge
    let pointP = Point {
        x: 2339,
        y: 2213,
        field: 9739,
        curve: ellipticCurve,
    };

    pointP.verificate_elliptic_curve();

    let pointQ = pointP.mul(3);
    let point3P = pointP.add(&(pointP.add(&pointP)));
    let point2P = pointP.add(&pointP);
    //pointQ.verificate_elliptic_curve();
    println!("2P : ({},{})", point2P.x, point2P.y);
    
    point3P.verificate_elliptic_curve();
    println!("pointQ : ({},{}) with field {} and elliptic curve Y^2 = X^3 + {} X + {}", pointQ.x, pointQ.y, pointQ.field, pointQ.curve.a, pointQ.curve.b);
    println!("Y^2 = {}", (pointQ.y * pointQ.y) % pointQ.field);
    println!("other part = {}", (pointQ.x.pow(3) + pointQ.curve.a * pointQ.x + pointQ.curve.b) % pointQ.field);

    println!("point3P : ({},{}) with field {} and elliptic curve Y^2 = X^3 + {} X + {}", point3P.x, point3P.y, point3P.field, point3P.curve.a, point3P.curve.b);
    println!("Y^2 = {}", (point3P.y * point3P.y) % point3P.field);
    println!("other part = {}", (point3P.x.pow(3) + point3P.curve.a * point3P.x + point3P.curve.b) % point3P.field);
}
