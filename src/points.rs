
use glam::Vec3;
use fastrand::*;

#[allow(dead_code)]
pub fn make_random_intensities(quantity: usize) -> Vec<f32> {
    let mut rng: Rng = Rng::with_seed(0);
    let mut intensities = Vec::<f32>::with_capacity(quantity);
    for _ in 0..quantity {
        intensities.push(rng.f32());
    }

    intensities
}

#[allow(dead_code)]
pub fn make_random_points(quantity: usize) -> Vec<Vec3> {
    let mut rng: Rng = Rng::with_seed(0);
    let mut points = Vec::<Vec3>::with_capacity(quantity);
    for _ in 0..quantity {
        points.push(Vec3::new(
            rng.f32(),
            rng.f32(),
            rng.f32(),
        ) - Vec3{x: 0.5, y: 0.5, z: 0.5});
    }

    points
}