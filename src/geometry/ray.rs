use super::vec3::Vec3;

pub struct Ray {
    start: Vec3<f64>,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(start: Vec3<f64>, mut dir: Vec3<f64>) -> Self {
        dir.norm();
        Ray { start, dir }
    }
}