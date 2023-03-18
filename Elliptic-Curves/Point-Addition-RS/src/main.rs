use std::ops::Add;
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

pub trait Addition {
    fn add(&self, _: &Self) -> Self;
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {}) % {}", self.x, self.y, self.field)
    }
}


impl Addition for Point {

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
            if self.x == secondPoint.x && self.y == (0 - secondPoint.y).rem_euclid(secondPoint.field) {
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
                    let y_calculation = (secondPoint.y - self.y).rem_euclid(self.field);
                    let x_calculation = (secondPoint.x - self.x).rem_euclid(self.field);
                    gamma =  (y_calculation * mod_inv(x_calculation, self.field)).rem_euclid(self.field);
                }
                else {// if (self.x == secondPoint.x && self.y == secondPoint.y) {
                    gamma =  ((3 * self.x.pow(2) + self.curve.a) * mod_inv(2*self.y, self.field)).rem_euclid(self.field);
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
    let pointA = Point {
        x: 5274,
        y: 2841,
        field: 9739,
        curve: ellipticCurve,
    };
    let pointB = Point {
        x: 8669,
        y: 740,
        field: 9739,
        curve: ellipticCurve,
    };

    // Verify elliptic curve for both point
    pointA.verificate_elliptic_curve();
    pointB.verificate_elliptic_curve();

    // do the addition
    let pointC = pointA.add(&pointB);
    let pointD = pointA.add(&pointA);

    println!("({},{}) + ({},{}) = ({},{})", pointA.x, pointA.y, pointB.x, pointB.y, pointC.x, pointC.y);
    println!("({},{}) + ({},{}) = ({},{})", pointA.x, pointA.y, pointA.x, pointA.y, pointD.x, pointD.y);

    // Exercise solution
    let P = Point {
        x: 493,
        y: 5564,
        field: 9739,
        curve: ellipticCurve,
    };

    let Q = Point {
        x: 1539,
        y: 4742,
        field: 9739,
        curve: ellipticCurve,
    };

    let R = Point {
        x: 4403,
        y: 5202,
        field: 9739,
        curve: ellipticCurve,
    };

    let  pointResult = P.add(&P.add(&Q.add(&R)));
    pointResult.verificate_elliptic_curve();
    println!("P + P + Q + R = ({},{})", pointResult.x, pointResult.y);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinity_point_addition() {
        let ellipticCurve = EllipticCurve {
            a: 497,
            b: 1768,
            o: 0,
        };
        let P = Point {
            x: 0,
            y: 0,
            field: 9739,
            curve: ellipticCurve,
        };
        let Q = Point {
            x: 493,
            y: 5564,
            field: 9739,
            curve: ellipticCurve,
        };
        Q.verificate_elliptic_curve();
        let R1 = Q.add(&P);
        let R2 = P.add(&Q);
        R1.verificate_elliptic_curve();
        R2.verificate_elliptic_curve();
        assert!(R1 == R2);
        assert!(R1 == Q);
    }

    #[test]
    fn test_y1_equals_minus_y2() {
        let ellipticCurve = EllipticCurve {
            a: 497,
            b: 1768,
            o: 0,
        };
        let P = Point {
            x: 493,
            y: 5564,
            field: 9739,
            curve: ellipticCurve,
        };
        let Q = Point {
            x: 493,
            y: (0 - P.y).rem_euclid(P.field),
            field: 9739,
            curve: ellipticCurve,
        };
        let O = Point {
            x: 0,
            y: 0,
            field: 9739,
            curve: ellipticCurve,
        };
        P.verificate_elliptic_curve();
        Q.verificate_elliptic_curve();
        let R1 = P.add(&Q);
        println!("R1 = {}", R1);
        assert!(R1 == O);
    }

    #[test]
    fn test_add_p_to_p2() {
        let ellipticCurve = EllipticCurve {
            a: 497,
            b: 1768,
            o: 0,
        };
        let P = Point {
            x: 493,
            y: 5564,
            field: 9739,
            curve: ellipticCurve,
        };
        P.verificate_elliptic_curve();
        let R = P.add(&P);
        R.verificate_elliptic_curve();
        println!("R = {}", R);
    }

    #[test]
    fn test_add_p_to_p() {
        let ellipticCurve = EllipticCurve {
            a: 497,
            b: 1768,
            o: 0,
        };
        let P = Point {
            x: 493,
            y: 5564,
            field: 9739,
            curve: ellipticCurve,
        };
        P.verificate_elliptic_curve();
        let R = P.add(&P);
        R.verificate_elliptic_curve();
        println!("R = {}", R);
    }

}
