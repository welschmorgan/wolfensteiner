use std::f32::consts::PI;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::slice::IterMut;

use super::Error;

#[derive(Clone, Copy, Default)]
pub struct Vec2<T: Copy = f32> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: Copy + Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Copy> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    
    pub const fn scalar(v: f32) -> Vec2 {
        Vec2::new(v, v)
    }
}

impl<T: Copy + Default> Vec2<T> {
    pub fn zero() -> Self {
        Self{x: Default::default(), y: Default::default()}
    }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Vec2<T> {
    pub fn sq_magn(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

macro_rules! impl_sqrt {
    ( $( $typs:ty ),+ ) => {
            $(
                impl Sqrt for $typs {
                fn sqrt(&self) -> Self {
                    (*self as f64).sqrt() as Self
                }
            }
        )+
    };
}

impl_sqrt!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<
        T: Copy + Default + PartialOrd + Mul<T, Output = T> + DivAssign<T> + Add<T, Output = T> + Sqrt,
    > Vec2<T>
{
    pub fn magn(&self) -> T {
        self.sq_magn().sqrt()
    }

    pub fn normalize(&mut self) -> T {
        let len = self.magn();
        if len > Default::default() {
            self.x /= len;
            self.y /= len;
        }
        len
    }

    pub fn normalized(&self) -> Self {
        let mut ret = *self;
        ret.normalize();
        ret
    }
}

pub trait ToPrimitive {
    fn to_primitive(&self) -> f64;
}

pub trait FromPrimitive<T: ToPrimitive> {
    fn from_primitive(v: T) -> Self;
}

macro_rules! impl_to_primitive {
    ( $( $typs: ty ),+ ) => {
        $(
            impl ToPrimitive for $typs {
                fn to_primitive(&self) -> f64 {
                    *self as f64
                }
            }

            impl<T: ToPrimitive> FromPrimitive<T> for $typs {
                fn from_primitive(v: T) -> Self {
                    let tmp = v.to_primitive();
                    tmp as Self
                }
            }
        )+
    };
}

impl_to_primitive!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Copy + ToPrimitive> Vec2<T> {
    pub fn cast<To: Copy + ToPrimitive + FromPrimitive<f64>>(&self) -> Vec2<To> {
        Vec2::new(
            To::from_primitive(self.x.to_primitive()),
            To::from_primitive(self.y.to_primitive()),
        )
    }
}

impl<T: Copy + MulAssign<T>> Vec2<T> {
    pub fn scaled(&self, by: T) -> Self {
        let mut ret = *self;
        ret.scale(by);
        return ret;
    }

    pub fn scale(&mut self, by: T) -> &mut Self {
        self.x *= by;
        self.y *= by;
        self
    }
}

impl<T: Copy + AddAssign> Add<Self> for Vec2<T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: Copy + AddAssign> AddAssign<Self> for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + SubAssign> Sub<Self> for Vec2<T> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: Copy + SubAssign> SubAssign<Self> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + MulAssign> Mul<Self> for Vec2<T> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<T: Copy + MulAssign> MulAssign<Self> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Copy + DivAssign> Div<Self> for Vec2<T> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}
impl<T: Copy + DivAssign> DivAssign<Self> for Vec2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<
        T: Copy + Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + FromPrimitive<f32>,
    > Vec2<T>
{
    pub fn rotate(&mut self, angle_degrees: f32) -> &mut Self {
        let angle_radians = angle_degrees * PI / 180.0;

        let cos_theta = T::from_primitive(angle_radians.cos());
        let sin_theta = T::from_primitive(angle_radians.sin());

        self.x = self.x * cos_theta - self.y * sin_theta;
        self.y = self.x * sin_theta + self.y * cos_theta;
        self
    }

    pub fn rotated(&self, angle_degrees: f32) -> Vec2<T> {
        let mut ret = *self;
        ret.rotate(angle_degrees);
        ret
    }
}

impl<T: Copy + PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Copy + PartialOrd> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl<T: Copy + Eq> Eq for Vec2<T> {}

impl<T: Copy + Ord> Ord for Vec2<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.y.cmp(&other.y)
    }
}

pub struct Ray<T: Copy> {
    start: Vec2<T>,
    direction: Vec2<T>,
    sample_id: T,
    sample_step: T,
    sample_max: T,
}

impl<
        T: Copy
            + PartialEq
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + MulAssign<T>
            + AddAssign<T>
            + Default
            + DivAssign
            + Mul
            + PartialOrd
            + Sqrt
            + ToPrimitive
            + FromPrimitive<f64>,
    > Ray<T>
{

    pub fn new(start: Vec2<T>, mut direction: Vec2<T>, sample_max: T, sample_step: T) -> crate::Result<Self> {
        if sample_max == T::default() {
            return Err(Error::OutOfBounds {
                value: i32::from_primitive(sample_max.to_primitive()),
                range: 1i32..usize::MAX as i32,
            });
        }
        direction.normalize();
        Ok(Self {
            start,
            direction,
            sample_step,
            sample_id: T::default(),
            sample_max,
        })
    }

    pub fn reset(&mut self) {
        self.sample_id = T::default()
    }
    
    pub fn samples(&mut self) -> Vec<Vec2<T>> {
        let mut ret = vec![];
        while let Some(sample) = self.next() {
            ret.push(sample);
        }
        ret
    }

}

impl<
        T: Copy
            + PartialEq
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + MulAssign<T>
            + AddAssign<T>
            + Default
            + DivAssign
            + Mul
            + PartialOrd
            + Sqrt
            + ToPrimitive
            + FromPrimitive<f64>,
    > Iterator for Ray<T>
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_id >= self.sample_max {
            return None;
        }
        self.sample_id += T::from_primitive(1.to_primitive());
        let ret =
            self.start + self.direction * Vec2::new(self.sample_id * self.sample_step, self.sample_id * self.sample_step).cast::<T>();
        Some(ret)
    }
}

pub struct Rect<T: Copy> {
    start: Vec2<T>,
    end: Vec2<T>
}

impl<T: Copy + Debug> Debug for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect").field("start", &self.start).field("end", &self.end).finish()
    }
}
impl<T: Copy> Rect<T> {
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self {
            start,
            end
        }
    }
}

impl<T: Copy + SubAssign<T>> Rect<T> {
    pub fn area(&self) -> Vec2<T> {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use super::{Ray, Vec2};

    #[test]
    fn ray() {
        let mut r: Ray<f32> = Ray::new(Vec2::zero(), Vec2::new(1.0, 1.0), 3f32, 0.5).unwrap();
        let samples = r.samples();
        println!("samples: {:#?}", samples);
        assert_eq!(samples.len(), 3)
    }
}