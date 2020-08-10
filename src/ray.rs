use super::scene_object::SceneObject;
use crate::{color::Color, vec3::Vec3};

const MAX_STEPS: u32 = 200;
const HIT_THRESHOLD: f64 = 1E-4;

#[derive(Debug)]
pub struct RayResult {
    /// Length in units of the ray
    pub len: f64,
    /// Where the ray intersected the scene in space
    pub hit_point: Vec3,
}

/// Performs the raymarching algorithm on a scene.
///
/// # Arguments
/// * `object`: the object/scene to calculate ray intersection with
/// * `point`: the origin point of this ray
/// * `direction`: direction vector of this ray. This does not need to be normalized.
/// * `backplanes`: if the ray ever reaches outside of the cube bounded by +/- backplanes,
/// it will be assumed to be a miss. This is a sort of culling mechanism.
/// * `t`: the 4th dimension (or other varied value) to pass to the object for animation purposes.
///
/// # Returns
/// `Some(result)` if the ray hit an object in the scene
/// `None` if the ray did not hit anything or hit one of the backplanes
pub fn cast_ray<C: Color, O: SceneObject<C>>(
    object: &O,
    point: Vec3,
    dir: Vec3,
    backplanes: Vec3,
    t: f64,
) -> Option<RayResult> {
    let dir = dir.normalized();
    let mut current_point = point.clone();
    let mut iterations = 0u32;
    let mut ray_len = 0.0;

    loop {
        let radius = object.distance_to(current_point, t);
        ray_len += radius;
        iterations += 1;
        current_point = point + ray_len * dir;
        if radius < HIT_THRESHOLD {
            return Some(RayResult {
                len: ray_len,
                hit_point: current_point,
            });
        }

        if iterations > MAX_STEPS {
            return None;
        }

        if current_point.x.abs() > backplanes.x
            || current_point.y.abs() > backplanes.y
            || current_point.z.abs() > backplanes.z
        {
            return None;
        }
    }
}
