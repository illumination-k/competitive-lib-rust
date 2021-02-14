use std::ops::*;
use num_traits::{Float, NumAssign, NumOps};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(xy: (T, T)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl<T> Point<T>
    where T: Float
{
    /// roation point  
    /// 0 <= theta <= 360
    pub fn rotaion(&self, theta: T) -> Self {
        let rasian: T = theta / T::from(180).unwrap() * T::from(std::f64::consts::PI).unwrap();
        let cos = rasian.cos();
        let sin = rasian.sin();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos
        }
    }
}

pub fn triangle_space<T>(o: Point<T>, a: Point<T>, b: Point<T>) -> T
    where T: Float
{
    let oa = a - o;
    let ob = b - o;
    (oa.x * ob.y - oa.y * ob.x).abs() / T::from(2).unwrap()
}

pub fn gradient<T>(a: Point<T>, b: Point<T>) -> Option<T>
    where T: Float
{
    let x = b - a;
    if x.x == T::zero() {
        return None
    }

    Some(x.y / x.x)
}

impl<T> Add<T> for Point<T>
    where T: Copy + NumOps
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl<T> AddAssign<T> for Point<T>
    where T: Copy + NumAssign
{
    fn add_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub<T> for Point<T>
    where T: Copy + NumOps
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> SubAssign<T> for Point<T>
    where T: Copy + NumAssign
{
    fn sub_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}


impl<T> Mul<T> for Point<T>
    where T: Copy + NumOps
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Point<T>
    where T: Copy + NumAssign
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Point<T>
    where T: Copy + NumOps
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Point<T>
    where T: Copy + NumAssign
{
    fn div_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Add<Point<T>> for Point<T>
    where T: NumOps
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
} 

impl<T> AddAssign<Point<T>> for Point<T>
    where T: Copy + NumAssign
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> Sub<Point<T>> for Point<T>
    where T: NumOps
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> SubAssign<Point<T>> for Point<T>
    where T: Copy + NumAssign
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> Mul<Point<T>> for Point<T>
    where T: NumOps
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> MulAssign<Point<T>> for Point<T>
    where T: Copy + NumAssign
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> Div<Point<T>> for Point<T>
    where T: NumOps
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T> DivAssign<Point<T>> for Point<T>
    where T: Copy + NumAssign
{
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::AbsDiffEq;

    impl<T: AbsDiffEq> AbsDiffEq for Point<T> where
        T::Epsilon: Copy,
    {
        type Epsilon = T::Epsilon;

        fn default_epsilon() -> T::Epsilon {
            T::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
            T::abs_diff_eq(&self.x, &other.x, epsilon) &&
            T::abs_diff_eq(&self.y, &other.y, epsilon)
        }
    }

    #[test]
    fn test_add() {
        let mut p1 = Point::new(0, 1);
        let p2= Point::new(1, 2);
        assert_eq!(p1+p2, Point::new(1, 3));

        p1 += 1;
        assert_eq!(p1, Point::new(1, 2));
        p1 += p2;
        assert_eq!(p1, Point::new(2, 4));
    }

    #[test]
    fn test_sub() {
        let mut p1 = Point::new(0, 1);
        let p2 = Point::new(1, 2);
        assert_eq!(p1-p2, Point::new(-1, -1));

        p1 -= 1;
        assert_eq!(p1, Point::new(-1, 0));
        p1 -= p2;
        assert_eq!(p1, Point::new(-2, -2));
    }

    #[test]
    fn test_mul() {
        let mut p1 = Point::new(0., 1.);
        let p2 = Point::new(1., 2.);
        assert_abs_diff_eq!(p1 * p2, Point::new(0., 2.));
        p1 *= 1.5;
        assert_abs_diff_eq!(p1, Point::new(0., 1.5));
        p1 *= p2;
        assert_abs_diff_eq!(p1, Point::new(0., 3.))
    }

    #[test]
    fn test_div() {
        let mut p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(2.0, 3.0);

        assert_abs_diff_eq!(p1 / p2, Point::new(1.0 / 2.0, 2.0 / 3.0));
        p1 /= 2.;
        assert_abs_diff_eq!(p1, Point::new(0.5, 1.));
        p1 /= p2;
        assert_abs_diff_eq!(p1, Point::new(0.5 / 2., 1. / 3.))
    }

    #[test]
    fn test_rotation() {
        let p1 = Point::new(1., -3.0.sqrt());
        assert_abs_diff_eq!(p1.rotaion(60.), Point::new(2., 0.));
    }
}