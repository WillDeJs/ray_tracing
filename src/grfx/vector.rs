use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3D(f32, f32, f32);

/// Implement simple Vec3D operations, add, neg, length, etc, dot, cross.
/// Suported operations:
///     retrieve x,y and z components separatey:
///         Vec3D.x()
///         Vec3D.y()
///         Vec3D.z()
///     add/substract two vectors
///         let new_vector = vecta + vectb;
///     multiply/divide with scalar
///         let new_vector = vect_a * 2.0;
///         let new_vector = vect_a / 2.0;
///      Overrriden operators:
///         *
///         /
///         -
///         +
///         ~ (neg)
///         *=
///         /=
///         +=
///         -=
///
impl Vec3D {
    /// Constructor
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    ///
    /// Create a vector from two points an origin and a destination
    ///  returns destination - origin
    ///
    pub fn from_points(origin: Point3D, dest: Point3D) -> Self {
        dest - origin
    }
    /// Retrieve x component
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Retrieve y component
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Retrieve z component
    pub fn z(&self) -> f32 {
        self.2
    }

    /// Calculate the squared length/magnitude of the Vec3D
    pub fn squared_length(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    /// Calculate the lenght/magnitude of the Vec3D
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    /// Turn the Vec3D into a unit Vec3D/
    /// This modifies Vec3D in place
    /// To generate new Vec3D see:backtrace unit()
    pub fn make_unit(&mut self) {
        let den = self.length();
        self.0 /= den;
        self.1 /= den;
        self.2 /= den;
    }

    /// Determine unit Vec3D (return new Vec3D)
    /// To turn this Vec3D into a unit Vec3D see: make_unit()
    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    /// Calculate dot product
    pub fn dot(left: &Self, right: &Self) -> f32 {
        left.0 * right.0 + left.1 * right.1 + left.2 * right.2
    }
    /// Calculate doct product
    /// ai + bj + ck
    /// wi + yj + zk
    ///
    /// i (bj*zk - yj*ck) - j(ai*zk - wk * ck) + k (ai * yj -wi*bj)
    ///
    pub fn cross(left: &Self, right: &Self) -> Self {
        Self(
            left.1 * right.2 - right.1 * left.2,
            -(left.0 * right.2 - right.0 * left.2),
            left.0 * right.1 - right.0 * left.1,
        )
    }
}
/* Operator overloading */

/// Operator +
impl Add for Vec3D {
    type Output = Vec3D;
    fn add(self, other: Vec3D) -> Self::Output {
        Vec3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
/// Operator -
impl Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, other: Vec3D) -> Self::Output {
        Vec3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
/// Operator  -(a) = -1
impl Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Self::Output {
        Vec3D(-self.0, -self.1, -self.2)
    }
}

/// Operator  +=
impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Vec3D) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

/// Operator  -=
impl SubAssign for Vec3D {
    fn sub_assign(&mut self, other: Vec3D) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

/// Operator  * (vect * number)
impl Mul<f32> for Vec3D {
    type Output = Vec3D;
    fn mul(self, scalar: f32) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

/// Operator  / (vect / number)
impl Div<f32> for Vec3D {
    type Output = Vec3D;
    fn div(self, scalar: f32) -> Self::Output {
        assert_ne!(scalar, 0.0);
        Self(self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}

/// Operator  * (vect * number) same as above but for references
impl Mul<f32> for &Vec3D {
    type Output = Vec3D;
    fn mul(self, scalar: f32) -> Self::Output {
        Vec3D(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

/// Operator  / (vect / number) same as above but for references
impl Div<f32> for &Vec3D {
    type Output = Vec3D;
    fn div(self, scalar: f32) -> Self::Output {
        assert_ne!(scalar, 0.0);
        Vec3D(self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}
impl MulAssign<f32> for Vec3D {
    /// Operator  *= ( Vec3D *= number)
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
        self.1 *= scalar;
        self.1 *= scalar;
    }
}

impl DivAssign<f32> for Vec3D {
    /// Operator  /= ( Vec3D /= number)
    fn div_assign(&mut self, scalar: f32) {
        assert_ne!(scalar, 0.0);
        self.0 /= scalar;
        self.1 /= scalar;
        self.1 /= scalar;
    }
}

/// Operator  * (number * Vec3D)
impl Mul<Vec3D> for f32 {
    type Output = Vec3D;
    fn mul(self, other: Vec3D) -> Self::Output {
        other * self
    }
}

/// Operator  * (number * Vec3D)
impl Mul<&Vec3D> for f32 {
    type Output = Vec3D;
    fn mul(self, other: &Vec3D) -> Self::Output {
        other.clone() * self
    }
}

///
/// Point structure to be used by vectors and other calcualations.
///
/// Offers operations to add/subtract points.
///
///
pub struct Point3D(f32, f32, f32);

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

/* Operator overloading for point*/

/// Operator -
impl Sub for Point3D {
    type Output = Vec3D;
    fn sub(self, other: Point3D) -> Self::Output {
        Vec3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
/// Operator  -(a) = -1
impl Neg for Point3D {
    type Output = Point3D;
    fn neg(self) -> Self::Output {
        Point3D(-self.0, -self.1, -self.2)
    }
}

/// Operator + (vector + point) -> point
impl Add<Point3D> for Vec3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Self::Output {
        Point3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

/// Operator + (point + vector) -> point
impl Add<Vec3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Vec3D) -> Self::Output {
        Point3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
