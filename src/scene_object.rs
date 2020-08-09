use crate::{color::Color, vec3::Vec3};

const NORMAL_EPSILON: f64 = 1E-7;

// t is the varied parameter (slicing in time dimension)
pub trait SceneObject<C> {
    fn distance_to(&self, point: Vec3, t: f64) -> f64;
    fn get_color(&self, t: f64) -> C;
    fn normal(&self, p: Vec3, t: f64) -> Vec3 {
        let x_plus = self.distance_to((p.x + NORMAL_EPSILON, p.y, p.z).into(), t);
        let x_minus = self.distance_to((p.x - NORMAL_EPSILON, p.y, p.z).into(), t);
        let y_plus = self.distance_to((p.x, p.y + NORMAL_EPSILON, p.z).into(), t);
        let y_minus = self.distance_to((p.x, p.y - NORMAL_EPSILON, p.z).into(), t);
        let z_plus = self.distance_to((p.x, p.y, p.z + NORMAL_EPSILON).into(), t);
        let z_minus = self.distance_to((p.x, p.y, p.z - NORMAL_EPSILON).into(), t);

        // dbg!(Vec3::from((p.x, p.y + NORMAL_EPSILON, p.z)));
        // dbg!(Vec3::from((p.x, p.y - NORMAL_EPSILON, p.z)));
        // dbg!(p);
        // dbg!(y_plus, y_minus);
        let x = x_plus - x_minus;
        let y = y_plus - y_minus;
        let z = z_plus - z_minus;
        Vec3 { x, y, z }.normalized()
    }
}

pub struct Sphere<C> {
    pub center: Vec3,
    pub radius: f64,
    pub color: C,
}

impl<C: Color> SceneObject<C> for Sphere<C> {
    fn distance_to(&self, point: Vec3, _: f64) -> f64 {
        let point = Vec3 {
            x: point.x % 5.0,
            y: point.y,
            z: point.z,
        };
        (point - self.center).magnitude() - self.radius
    }

    fn get_color(&self, _: f64) -> C {
        self.color
    }
}
