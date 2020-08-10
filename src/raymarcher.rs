use crate::{color::Color, ray::cast_ray, scene_object::SceneObject, vec3::Vec3};

#[derive(Debug)]
pub struct RayMarcherConfig<C> {
    /// Width of the rendered image in px
    pub width: usize,
    /// Height of the rendered image in px
    pub height: usize,
    /// Camera position in 3d space
    pub camera_pos: Vec3,
    /// Point that the camera should point towards (usually leave this at (0, 0, 0) for julia sets)
    pub look_at: Vec3,
    /// Position of the Phong directional light in 3d space
    pub light_pos: Vec3,
    /// Color to render if a ray missed the scene
    pub background_color: C,
    /// Zoom level of the camera. 1.0 normal zoom
    pub camera_zoom: f64,
    /// Size of subpixel grid. anti_aliasing_level of 4 will create a 4x4 subpixel grid (so 16x AA)
    pub anti_aliasing_level: u32,
    /// Position of the back culling planes. Any rays that hit this will be assumed to be a miss
    pub backplane_positions: Vec3,
    /// Phong shininess constant
    pub specular_shininess: f64,
    /// Color of specular highlights
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
    /// Object or scene to render
    pub object: O,
    /// Configuration to run under. see `RayMarcherConfig`
    pub config: RayMarcherConfig<C>,
}

impl<C: Color, O: SceneObject<C>> RayMarcher<C, O> {
    /// Marches a ray (and secondary rays) to get a final color.
    /// Will send multiple rays if anti-aliasing is enabled, and average them.
    ///
    /// `x` and `y` are the pixel locations. They must be with the width and height of the configuration.
    ///
    /// `t` is the varied parameter, used for animation.
    ///
    /// Returns the traced color of the pixel.
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
                pixel_sum =
                    pixel_sum + self.trace_with_lighting(self.config.camera_pos, ray_dir, t);
            }
        }
        pixel_sum * (1.0 / (aa_level * aa_level) as f64)
    }

    /// Trace a ray with point and direction, calculate lighting for this ray, and return the color.
    fn trace_with_lighting(&self, point: Vec3, dir: Vec3, t: f64) -> C {
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

                self.object.get_color(t) * s_dot_n + self.config.specular_color * specular_term
            }
            None => self.config.background_color,
        }
    }

    /// Given a pixel x and y, calculate the direction of the needed ray.
    /// Takes camera position, zoom, and look_at from the configuration in to account.
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
