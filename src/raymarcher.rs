use crate::{scene_object::SceneObject, vec3::Vec3, ray::cast_ray};

pub struct RayMarcher<O: SceneObject> {
    pub object: O,
    pub config: RayMarcherConfig,
}

impl<O: SceneObject> RayMarcher<O> {
    pub fn get_pixel_color(&self, (x, y): (usize, usize), (width, height): (usize, usize), t: f64) -> Vec3 {
        let aa_level = self.config.anti_aliasing_level;

        let subpixel_size = 1.0 / aa_level as f64;
        let mut pixel_sum = Vec3::default();
        for subpixel_x in 0..aa_level {
            for subpixel_y in 0..aa_level {
                let ray_dir = Self::camera_ray_direction(
                    x as f64 + subpixel_x as f64 * subpixel_size,
                    y as f64 + subpixel_y as f64 * subpixel_size,
                    self.config.camera_pos,
                    self.config.look_at,
                    self.config.camera_zoom,
                    (width, height),
                );
                pixel_sum = pixel_sum + self.trace(self.config.camera_pos, ray_dir, t);
            }
        }
        (1.0 / (aa_level * aa_level) as f64) * pixel_sum
    }

    fn trace(&self, point: Vec3, dir: Vec3, t: f64) -> Vec3 {
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

                s_dot_n * self.object.get_color(t) + specular_term * self.config.specular_color
            }
            None => self.config.background_color
        }
    }

    fn camera_ray_direction(x: f64, y: f64, cam_pos: Vec3, look_at: Vec3, zoom: f64, (width, height): (usize, usize)) -> Vec3 {
        let u = -(x as f64 / width as f64 * 2.0 - 1.0);
        let v = y as f64 / height as f64 * 2.0 - 1.0;

        let look_dir = (look_at - cam_pos).normalized();
        let right_vec = Vec3::from((0, -1, 0)).cross(look_dir).normalized();
        let down_vec = look_dir.cross(right_vec).normalized();

        let zoomed_cam_pos = cam_pos + zoom * look_dir;
        let pix_pos = zoomed_cam_pos + u * right_vec + v * down_vec;
        let dir = pix_pos - cam_pos;
        dir.normalized()
    }
}

#[derive(Debug)]
pub struct RayMarcherConfig {
    pub camera_pos: Vec3,
    pub look_at: Vec3,
    pub light_pos: Vec3,
    pub background_color: Vec3,
    pub camera_zoom: f64,
    pub anti_aliasing_level: u32,
    pub backplane_positions: Vec3,
    pub specular_shininess: f64,
    pub specular_color: Vec3,
}

impl Default for RayMarcherConfig {
    fn default() -> Self {
        RayMarcherConfig {
            camera_pos: Vec3 { x: 2.0, y: 4.0, z: 4.0 },
            look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            light_pos: Vec3 { x: 2.0, y: 4.0, z: 4.0 },
            background_color: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            camera_zoom: 3.0,
            anti_aliasing_level: 4u32,
            backplane_positions: Vec3 { x: 3.0, y: 3.0, z: 3.0 },
            specular_shininess: 50.0,
            specular_color: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        }
    }
}