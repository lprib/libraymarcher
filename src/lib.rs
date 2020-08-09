pub mod color;
pub mod julia;
pub mod quaternion;
pub mod ray;
pub mod raymarcher;
pub mod scene_object;
pub mod vec3;

#[cfg(test)]
mod tests {
    use super::*;
    use quaternion::Quaternion;
    use raymarcher::RayMarcherConfig;

    #[test]
    fn test_output() {
        let w = 200;
        let h = 100;

        let config: RayMarcherConfig<f64> = RayMarcherConfig {
            width: w,
            height: h,
            camera_zoom: 3.0,
            anti_aliasing_level: 2,
            camera_pos: (2.0, 2.5, 2.5).into(),
            specular_shininess: 20.0,
            ..Default::default()
        };

        let obj = julia::Julia {
            c: Quaternion::new(-0.450, -0.447, 0.181, 0.306),
            color: 1.0,
        };
        let march = raymarcher::RayMarcher {
            object: obj,
            config: config,
        };
        let gradient = " .:-=+*#%@";

        for i in 0..h {
            for j in 0..w {
                let c = march.get_pixel_color(j, i, 0.0);
                let gray = if c < 0.0 {
                    0.0
                } else if c > 1.0 {
                    1.0
                } else {
                    c
                };
                let idx = (gray * (gradient.len() - 1) as f64).floor() as usize;
                // println!("{}", idx);
                print!("{}", gradient.chars().nth(idx).expect("fucked"));
            }
            println!();
        }
    }
}
