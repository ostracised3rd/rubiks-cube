

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}



impl Vector3 {
    // pub fn add(&mut self, other: &Vector3) {
    //     self.x += other.x;
    //     self.y += other.y;
    //     self.z += other.z;
    // }

    pub fn rotate(&mut self, ax: f64, ay: f64) {
        let mut m = vec![vec![self.x], vec![self.y], vec![self.z]];

        m = mat_mult(m, rotate_x(ax));
        m = mat_mult(m, rotate_y(ay));
        // let z = m[2][0];
        // m = mat_mult(m, perspective(150., z));
        
        self.x = m[0][0];
        self.y = m[1][0];
        self.z = m[2][0];
    }
}



pub fn add_vector(a: &Vector3, b: &Vector3) -> Vector3 {
    Vector3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}


pub fn mat_mult(m2: Vec<Vec<f64>>, m1: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

        // println!("m2: {:?}, m1: {:?}", m2, m1);

        let ca = m1[0].len();
        let ra = m1.len();
        let cb = m2[0].len();
        let rb = m2.len();

        if ca != rb {
            panic!("what just happened");
        }
        
        let mut res: Vec<Vec<f64>> = Vec::new();

        for j in 0..ra {
            res.push(Vec::new());
            for i in 0..cb {
                let mut sum: f64 = 0.;
                for n in 0..ca {
                    sum += m1[j][n] * m2[n][i];
                }
                res[j].push(sum);
            }
        }

        return res
}


// fn matrix_mult(point: &Vector3, matrix: Vec<Vector3>) -> Vector3 {
//     Vector3 { 
//         x: point.x * matrix[0].x + point.y * matrix[1].x + point.z * matrix[2].x, 
//         y: point.x * matrix[0].y + point.y * matrix[1].y + point.z * matrix[2].y, 
//         z: point.x * matrix[0].z + point.y * matrix[1].z + point.z * matrix[2].z 
//     }
// }

fn rotate_x(a: f64) -> Vec<Vec<f64>> {
    vec![
        vec![1., 0.,      0.],
        vec![0., a.cos(), -a.sin()],
        vec![0., a.sin(), a.cos()],
    ]
}

fn rotate_y(a: f64) -> Vec<Vec<f64>> {
    vec![
        vec![a.cos(),  0.,a.sin()],
        vec![0.,       1.,0.],
        vec![-a.sin(), 0.,a.cos()],
    ]
}

pub fn perspective(dist: f64, z: f64) -> Vec<Vec<f64>> {
    let psp = 1. / (dist - z) ;

    vec![
        vec![psp , 0., 0.],
        vec![0., psp , 0.],
        vec![0., 0.  , 1.],
    ]
}


// fn rotate_z(a: f32) -> Vec<Vec<f32>> {
//     vec![
//         vec![a.cos(), -a.sin(), 0.],
//         vec![a.sin(), a.cos(),  0.],
//         vec![0.,      0.,       1.],
//     ]
// }





// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Vector {
//     pub x: f64,
//     pub y: f64,
// }

// impl Vector {


//     pub fn add(&mut self, other: Vector) {
//         self.x =  self.x + other.x;
//         self.y =  self.y + other.y;
//     }

//     pub fn mag(&self) -> f64 {
//         (self.x.powf(2.) + self.y.powf(2.)).sqrt()
//     }

//     pub fn set_mag(&mut self, new_mag: f64) {
//         let mag = self.mag();
//         if mag == 0. {return}

//         self.x = self.x * new_mag / mag;
//         self.y = self.y * new_mag / mag;

//         // println!("{:?}", self);
//     }

//     pub fn sub(&mut self, other: Vector) {
//         self.x = self.x - other.x;
//         self.y = self.y - other.y;
//     }

//     // pub fn mult(&mut self, scaler: i32) {
//     //     self.x *= scaler;
//     //     self.y *= scaler;
//     // }

//     // pub fn div(&mut self, scaler: i32) {
//     //     self.x /= scaler;
//     //     self.y /= scaler;
//     // }
// }

// pub fn add_vectors(a: Vector, b: Vector) -> Vector {
//     Vector { x: (a.x + a.x), y: (a.y + b.y) }
// }


// pub fn sub_vectors(a: Vector, b: Vector) -> Vector {
//     Vector { x: (a.x - b.x), y: (a.y - b.y) }
// }


// pub fn mag_vectors(a: Vector, b: Vector) -> f64 {
//     ((a.x + b.x).powf(2.) + (a.y + b.y).powf(2.)).sqrt()
// }