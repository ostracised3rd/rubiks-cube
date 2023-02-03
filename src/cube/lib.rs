use super::maths::Vector3;
use super::cube::Cube;

#[derive(Debug, Clone)]
pub struct World {
    pub center : Vector3,
    pub camera: Vector3,
    pub cube: Cube,
}

impl World {
    pub fn new() -> World {
        World {
            center: Vector3{x: 0., y: 0., z: 0.},
            camera: Vector3{x: 0., y: 0., z: 1.},
            cube: Cube::new(Vector3{x: 0., y: 0., z: 0.}, 10.),
        }
    }
}

