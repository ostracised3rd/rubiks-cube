use super::maths::{Vector3, add_vector, perspective, mat_mult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    GREEN=1,
    YELLOW=2,
    RED=3,
    BLUE=4,
    PURPLE=5,
    WHITE=6,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub points: Vec<Vector3>,
    pub color: Color,
    center: Vector3
}


impl Face {
    fn new(
        points: Vec<Vector3>, 
        color: Color, 
    ) -> Face {
        let mut center = points.iter().fold(Vector3 {x:0., y:0., z:0.}, |i, p| {add_vector(&i, p)});
        center.x = center.x / points.len() as f32;
        center.y = center.y / points.len() as f32;
        center.z = center.z / points.len() as f32;

        Face {
            points,
            color,
            center
        }
    }

    fn rotate(&mut self, ax: f32, ay: f32) {

        for v in self.points.iter_mut() {
            v.rotate(ax, ay)
        }

        self.find_center();
    }

    fn find_center(&mut self) {
        let center = self.points.iter().fold(Vector3 {x:0., y:0., z:0.}, |i, p| {add_vector(&i, p)});
        self.center.x = center.x / self.points.len() as f32;
        self.center.y = center.y / self.points.len() as f32;
        self.center.z = center.z / self.points.len() as f32;
    }

    fn data(&self, mag: f32) -> (Vec<(f32, f32, f32)>, Color, f32) {
        let d = self.points.iter().map(|v| {
            let mut m = vec![vec![v.x], vec![v.y], vec![v.z]];
            let z = v.z;
            m = mat_mult(m, perspective(150., z));
            println!("{:?}", &m);
            (m[0][0] * mag, m[1][0] * mag, m[2][0])
        }).collect::<Vec<(f32, f32, f32)>>();

        (d, self.color, self.center.z)
    }
}


pub struct Cube {
    pub angel: Vector3,
    pub faces: Vec<Face>,
    mag: f32,
}

impl Cube {

    pub fn new(center: Vec<f32>, mag: f32) -> Cube {

        let radius = 0.5;

        let p0 = Vector3 {x: center[0]-radius, y:center[1]+radius, z: center[2]+radius};
        let p1 = Vector3 {x: center[0]+radius, y:center[1]+radius, z: center[2]+radius};
        let p2 = Vector3 {x: center[0]+radius, y:center[1]-radius, z: center[2]+radius};
        let p3 = Vector3 {x: center[0]-radius, y:center[1]-radius, z: center[2]+radius};
        let p4 = Vector3 {x: center[0]-radius, y:center[1]+radius, z: center[2]-radius};
        let p5 = Vector3 {x: center[0]+radius, y:center[1]+radius, z: center[2]-radius};
        let p6 = Vector3 {x: center[0]+radius, y:center[1]-radius, z: center[2]-radius};
        let p7 = Vector3 {x: center[0]-radius, y:center[1]-radius, z: center[2]-radius};


        Cube {
            angel: Vector3 {x:0., y:0., z:0.},
            // radius,
            faces: vec![
                Face::new(vec![p0, p1, p2, p3], Color::BLUE,    ),
                Face::new(vec![p4, p5, p6, p7], Color::YELLOW,  ),
                Face::new(vec![p0, p4, p7, p3], Color::WHITE,   ),
                Face::new(vec![p1, p5, p6, p2], Color::RED,     ),
                Face::new(vec![p0, p4, p5, p1], Color::GREEN,   ),
                Face::new(vec![p3, p7, p6, p2], Color::PURPLE,  ),
            ],
            mag
        }
    }

    
    pub fn rotate(&mut self, ax: f32, ay: f32) {
        self.angel.x = (self.angel.x + ax + 360.) % 360.;
        self.angel.y = (self.angel.y + ay + 360.) % 360.;


        for f in self.faces.iter_mut() {
            f.rotate(ax, ay);
        }
    }


    pub fn visible(&self) -> Vec<(Vec<(f32, f32, f32)>, Color, f32)> {

        let mut v = Vec::new();
        for f in self.faces.iter() {
            if f.center.z > 0. {
                v.push(f.data(self.mag))
            }
        }

        v.sort_by(|a, b| {
            (a.2).partial_cmp(&b.2).unwrap()
        });

        return v
    }

}