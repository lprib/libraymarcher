use crate::{quaternion::Quaternion, scene_object::SceneObject, vec3::Vec3};

const MAX_ITERS: i32 = 20;

pub struct Julia {
    pub c: Quaternion,
    pub color: Vec3,
}

impl SceneObject for Julia {
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

    fn get_color(&self, t: f64) -> Vec3 {
        self.color
    }
}
