use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl UVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_i_position(&self) -> IVec2 {
        IVec2::new(self.x as isize, self.y as isize)
    }
}

impl Add<UVec2> for UVec2 {
    type Output = Self;

    fn add(self, rhs: UVec2) -> Self::Output {
        UVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<usize> for UVec2 {
    type Output = UVec2;

    fn mul(self, rhs: usize) -> Self::Output {
        UVec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<isize> for UVec2 {
    type Output = IVec2;

    fn mul(self, rhs: isize) -> Self::Output {
        IVec2::new(self.x as isize * rhs, self.y as isize * rhs)
    }
}

impl Sub<IVec2> for UVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new(self.x as isize - rhs.x, self.y as isize - rhs.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl IVec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn as_u_position(&self) -> UVec2 {
        UVec2::new(self.x as usize, self.y as usize)
    }

    pub fn is_coord_safe(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < width as isize && self.y >= 0 && self.y < height as isize
    }

    pub fn length(&self) -> isize {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)) as f64) as isize
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        IVec2::new(self.x / length, self.y / length)
    }
}

impl Add<IVec2> for UVec2 {
    fn add(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x as isize + rhs.x, self.y as isize + rhs.y)
    }

    type Output = IVec2;
}

impl Add<UVec2> for IVec2 {
    fn add(self, rhs: UVec2) -> IVec2 {
        IVec2::new(self.x + rhs.x as isize, self.y + rhs.y as isize)
    }

    type Output = IVec2;
}

impl Sub<IVec2> for IVec2 {
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<IVec2> for IVec2 {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div<IVec2> for IVec2 {
    type Output = Self;

    fn div(self, rhs: IVec2) -> Self::Output {
        IVec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Mul<usize> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as isize
    }
}

impl Mul<isize> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as isize
    }
}

#[cfg(test)]
mod math_tests {
    use crate::common::math::IVec2;

    #[test]
    fn vector_length() {
        assert_eq!(IVec2::new(6, 8).length(), 10)
    }
}

/* pub fn get_intersection(p0: IVec2, p1: IVec2, p2: IVec2, p3: IVec2) -> Option<IVec2> {
    let s1_x = p1.x - p0.x;
    let s2_x = p3.x - p2.x;

    let s1_y = p1.y - p0.y;
    let s2_y = p3.y - p2.y;

    let s = (-s1_y * (p0.x - p2.x) + s1_x * (p0.y - p2.y)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = (s2_x * (p0.y - p2.y) - s2_y * (p0.x - p2.x)) / (-s2_x * s1_y + s1_x * s2_y);

    if s >= 0 && s <= 1 && t >= 0 && t <= 1 {
        let x = p0.x + (t * s1_x);
        let y = p0.y + (t * s1_y);
        Some(IVec2::new(x, y))
    } else {
        None
    }
}

fn solve_lin_equation_float(a: f64, b: f64, c: f64, d: f64, u: f64, v: f64) -> (f64, f64) {
    if a.abs() > c.abs() {
    let f = u * c / a;
    let g = b * c / a;
    let y = (v - f) / (d - g);
    return ((f - g * y) / c, y);
         } else {
        let f = v * a / c;
        let g = d * a / c;
        let x = (u - f) / (b - g);
        return (x, (f - g * x) / a);
    }
}*/

#[inline]
pub fn mod_floor(a: isize, n: isize) -> isize {
    ((a % n) + n) % n
}