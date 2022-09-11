use std::ops::{Mul, Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T>(pub T, pub T, pub T);


impl Vec3<f64> {
    pub fn len_sq(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn norm(&mut self) {
        let len = self.len();
        self.0 /= len;
        self.1 /= len;
        self.2 /= len;
    }

    pub fn get_norm(&self) -> Self {
        let mut clone = *self;
        clone.norm();
        clone
    }
}

impl<T: Add<Output = T>> Add for Vec3<T>  {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub trait DotProd: Sized + Add<Output = Self> + Mul<Output = Self> {}

impl<T: Sized + Add<Output = Self> + Mul<Output = Self>> DotProd for T {}

impl<T: DotProd> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2  * rhs.2
    }
}
