use crate::{color::Color, ray::cast_ray, scene_object::SceneObject, vec3::Vec3};

#[derive(Debug)]
pub struct RayMarcherConfig<C> {
    pub width: usize,
    pub height: usize,
    pub camera_pos: Vec3,
    pub look_at: Vec3,
    pub light_pos: Vec3,
    pub background_color: C,
    pub camera_zoom: f64,
    pub anti_aliasing_level: u32,
    pub backplane_positions: Vec3,
    pub specular_shininess: f64,
    pub specular_color: C,
}

impl<C: Color> Default for RayMarcherConfig<C> {
    fn default() -> Self {
        RayMarcherConfig {
            width: 10,
            height: 10,
            camera_pos: Vec3 {
                x: 2.0,
                y: 4.0,
                z: 4.0,
            },
            look_at: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            light_pos: Vec3 {
                x: 2.0,
                y: 4.0,
                z: 4.0,
            },
            background_color: C::default(),
            camera_zoom: 3.0,
            anti_aliasing_level: 4u32,
            backplane_positions: Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
            specular_shininess: 50.0,
            specular_color: C::white(),
        }
    }
}

pub struct RayMarcher<C, O: SceneObject<C>> {
    pub object: O,
    pub config: RayMarcherConfig<C>,
}

impl<C: Color, O: SceneObject<C>> RayMarcher<C, O> {
    pub fn get_pixel_color(&self, x: usize, y: usize, t: f64) -> C {
        let aa_level = self.config.anti_aliasing_level;

        let subpixel_size = 1.0 / aa_level as f64;
        let mut pixel_sum = C::default();
        for subpixel_x in 0..aa_level {
            for subpixel_y in 0..aa_level {
                let ray_dir = self.camera_ray_direction(
                    x as f64 + subpixel_x as f64 * subpixel_size,
                    y as f64 + subpixel_y as f64 * subpixel_size,
                );
                pixel_sum = pixel_sum.add(self.trace(self.config.camera_pos, ray_dir, t));
            }
        }
        pixel_sum.mul(1.0 / (aa_level * aa_level) as f64)
    }

    fn trace(&self, point: Vec3, dir: Vec3, t: f64) -> C {
        let res = cast_ray(&self.object, point, dir, self.config.backplane_positions, t);
        //TODO unsure if this is necessary:
        let normal_backoff_dist = 1E-7;
        match res {
            Some(res) => {
                let len = (res.hit_point - self.config.camera_pos).magnitude();
                let len = (len / 2.0).powi(2);
                // return (len, len, len).into();
                // return self.object.get_color(t);

                // if there is a ray hit, do Phong lighting calculations
                let light_vec = (self.config.light_pos - res.hit_point).normalized();
                let norm_point = res.hit_point - normal_backoff_dist * dir;
                let norm = self.object.normal(norm_point, t);
                let s_dot_n = norm.dot(light_vec);
                // return norm;

                //specularity
                let reflect_vec = (-light_vec).reflect(norm).normalized();
                let view_vec = (self.config.camera_pos - res.hit_point).normalized();
                let r_dot_v = reflect_vec.dot(view_vec.normalized());
                let specular_term = r_dot_v.powf(self.config.specular_shininess);
                let specular_term = if r_dot_v > 0.0 { specular_term } else { 0.0 };

                self.object
                    .get_color(t)
                    .mul(s_dot_n)
                    .add(self.config.specular_color.mul(specular_term))
            }
            None => self.config.background_color,
        }
    }

    fn camera_ray_direction(&self, x: f64, y: f64) -> Vec3 {
        let u = -(x as f64 / self.config.width as f64 * 2.0 - 1.0);
        let v = y as f64 / self.config.height as f64 * 2.0 - 1.0;

        let look_dir = (self.config.look_at - self.config.camera_pos).normalized();
        let right_vec = Vec3::from((0, -1, 0)).cross(look_dir).normalized();
        let down_vec = look_dir.cross(right_vec).normalized();

        let zoomed_cam_pos = self.config.camera_pos + self.config.camera_zoom * look_dir;
        let pix_pos = zoomed_cam_pos + u * right_vec + v * down_vec;
        let dir = pix_pos - self.config.camera_pos;
        dir.normalized()
    }
}
