// use super::statics::{Face, Color, Category};
// use std::collections::HashMap;

use super::maths::Vector3;






#[derive(Debug, Clone)]
pub struct Cube {
    pub points: Vec<Vector3>,
    //    ful: Vector3,
    //    fur: Vector3,
    //    bul: Vector3,
    //    bur: Vector3,
    //    fdl: Vector3,
    //    fdr: Vector3,
    //    bdl: Vector3,
    //    bdr: Vector3,
}

impl Cube {
    pub fn new(center: Vector3, size: f64) -> Cube {
        Cube {
            points: vec![
                Vector3{x: center.x - size, y: center.y + size, z: center.z + size},
                Vector3{x: center.x + size, y: center.y + size, z: center.z + size},
                Vector3{x: center.x - size, y: center.y + size, z: center.z - size},
                Vector3{x: center.x + size, y: center.y + size, z: center.z - size},
                Vector3{x: center.x - size, y: center.y - size, z: center.z + size},
                Vector3{x: center.x + size, y: center.y - size, z: center.z + size},
                Vector3{x: center.x - size, y: center.y - size, z: center.z - size},
                Vector3{x: center.x + size, y: center.y - size, z: center.z - size},
            ],
        }
    }
}




// #[derive(Debug, Clone)]
// pub struct Surf {
//     pub face: Face,
//     pub color: Color
// }

// #[derive(Debug, Clone)]
// pub struct Block {
//     pub surfaces: Vec<Surf>,
//     pub cat: Category
// }


// #[derive(Debug, Clone)]
// pub struct Cube {
//     pub row1: [Block; 9],
//     pub row2: [Block; 9],
//     pub row3: [Block; 9],
//     pub col1: [Block; 9],
//     pub col2: [Block; 9],
//     pub col3: [Block; 9],
// }



// impl Cube {
//     pub fn new(_size: f64) -> Cube {
//         let blocks = load_blocks();
//         Cube{
//             row1: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//             row2: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//             row3: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//             col1: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//             col2: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//             col3: [
//                 blocks["r1c1"].clone(),
//                 blocks["r1c2"].clone(),
//                 blocks["r1c3"].clone(),
//                 blocks["r2c1"].clone(),
//                 blocks["r2c2"].clone(),
//                 blocks["r2c3"].clone(),
//                 blocks["r3c1"].clone(),
//                 blocks["r3c2"].clone(),
//                 blocks["r3c3"].clone(),
//             ],
//         }
//     } 
// }


// fn surfaces() -> HashMap<String, Surf> {

//     let mut surf = HashMap::<String, Surf>::new();
//     surf.insert(
//         "up".to_string(),
//         Surf{
//         face: Face::UP,
//         color: Color::RED,
//     });

//     surf.insert(
//         "left".to_string(),
//         Surf{
//         face: Face::LEFT,
//         color: Color::GREEN,
//     });
        
//     surf.insert(
//         "front".to_string(),
//         Surf{
//         face: Face::FRONT,
//         color: Color::YELLOW,
//     });

//     surf.insert(
//         "right".to_string(),
//         Surf{
//         face: Face::RIGHT,
//         color: Color::BLUE,
//     });

//     surf.insert(
//         "back".to_string(),
//         Surf{
//         face: Face::BACK,
//         color: Color::WHITE,
//     }); 

//     surf.insert(
//         "down".to_string(),
//         Surf{
//         face: Face::DOWN,
//         color: Color::PURPLE,
//     }); 

//     surf
// }



// fn load_blocks() -> HashMap<String, Block> {
//     let surf = surfaces();
//     let mut blocks: HashMap<String, Block> = HashMap::new();
//     blocks.insert(
//         "r1c1".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["left"].clone(),
//             surf["front"].clone(),
//         ],
//         cat: Category::CORNER
//     });
    
//     blocks.insert(
//         "r1c2".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["front"].clone(),
//         ],
//         cat: Category::EDGE
//     });

//     blocks.insert(
//         "r1c3".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["front"].clone(),
//             surf["right"].clone(),
//         ],
//         cat: Category::CORNER
//     });

//     blocks.insert(
//         "r2c1".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["left"].clone(),
//         ],
//         cat: Category::EDGE
//     });

//     blocks.insert(
//         "r2c2".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//         ],
//         cat: Category::CENTER
//     });

//     blocks.insert(
//         "r2c3".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["right"].clone(),
//         ],
//         cat: Category::EDGE
//     });

//     blocks.insert(
//         "r3c1".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["left"].clone(),
//             surf["back"].clone(),
//         ],
//         cat: Category::CORNER
//     });

//     blocks.insert(
//         "r3c2".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["back"].clone(),
//         ],
//         cat: Category::EDGE
//     });

//     blocks.insert(
//         "r3c3".to_string(),
//         Block{
//         surfaces: vec![
//             surf["up"].clone(),
//             surf["right"].clone(),
//             surf["back"].clone(),
//         ],
//         cat: Category::CORNER
//     });

//     blocks
// }
