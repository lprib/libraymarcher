use crate::{color::Color, quaternion::Quaternion, scene_object::SceneObject, vec3::Vec3};

/// The maximum number of escape iterations that are used for each distance estimation
const MAX_ITERS: i32 = 20;

/// A Julia set defined by a Quaternion c value (used in z = z^2 + c)
/// Includes a color, which is passed through to the raymarcher
pub struct Julia<C> {
    pub c: Quaternion,
    pub color: C,
}

impl<C: Color> SceneObject<C> for Julia<C> {
    fn distance_to(&self, point: Vec3, t: f64) -> f64 {
        let mut z = Quaternion::new(point.x, point.y, point.z, t);
        let mut dz = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        while count < MAX_ITERS {
            let z_new = z * z + self.c;
            dz = 2.0 * z * dz;
            z = z_new;

            if z.magnitude() > 4.0 {
                break;
            }
            count += 1;
        }

        let dist: f64 = z.magnitude() * z.magnitude().ln() / dz.magnitude();
        dist * 0.2
    }

    fn get_color(&self, t: f64) -> C {
        self.color
    }
}
