mod color;
mod julia;
mod ray;
mod raymarcher;
mod scene_object;
mod vec3;

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::Quaternion;
    use raymarcher::RayMarcherConfig;

    #[test]
    fn test_output() {
        let config = RayMarcherConfig {
            camera_zoom: 3.0,
            anti_aliasing_level: 2,
            camera_pos: (2.0, 2.5, 2.5).into(),
            specular_shininess: 20.0,
            ..Default::default()
        };

        let obj = julia::Julia {
            c: Quaternion::new(-0.450, -0.447, 0.181, 0.306),
            color: (1, 0, 0).into(),
        };
        let march = raymarcher::RayMarcher {
            object: obj,
            config: config,
        };

        let w = 200;
        let h = 100;
        let gradient = " .:-=+*#%@";

        for i in 0..h {
            for j in 0..w {
                let c = march.get_pixel_color((j, i), (w, h), 0.0);
                let gray = if c.x < 0.0 {
                    0.0
                } else if c.x > 1.0 {
                    1.0
                } else {
                    c.x
                };
                let idx = (gray * (gradient.len() - 1) as f64).floor() as usize;
                // println!("{}", idx);
                print!("{}", gradient.chars().nth(idx).expect("fucked"));
            }
            println!();
        }
    }
}
