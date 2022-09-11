use std::ops::Neg;

use super::vec3::{DotProd, Vec3};

pub struct Plane<T> {
    normal: Vec3<T>,
    dist: T,
}

impl Plane<f64> {
    pub fn new(point: Vec3<f64>, mut normal: Vec3<f64>) -> Self {
        normal.norm();
        Plane { normal: normal, dist: -(normal * point) }
    }
}
