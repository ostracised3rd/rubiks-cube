// use nannou::prelude::*;
mod cube;

// fn main() {
//     nannou::app(model)
//         .update(update)
//         .simple_window(view)
//         .run();
// }

// struct Model {
//     cube: cube_old::cube::Cube,
// }

// fn model(_app: &App) -> Model {
//     Model {
//         cube: cube_old::cube::Cube::new(vec![0., 0., 0.,], 300.) 
//     }
// }

// fn update(_app: &App, model: &mut Model, _update: Update) {
//     model.cube.rotate(0.01, 0.005);
// }

// fn view(app: &App, model: &Model, frame: Frame){
//     frame.clear(SLATEGREY);

//     let draw = app.draw();

    

//     for (ps, color, _) in model.cube.visible() {
//         let pts: Vec<Point2> = ps.iter().map(|p| pt2(p.0, p.1)).collect();

//         draw.quad()
//             // .no_fill()
//             // .stroke_weight(2.)
//             .color(match color {
//                 cube_old::cube::Color::GREEN    => YELLOWGREEN,
//                 cube_old::cube::Color::YELLOW   => DARKORANGE,
//                 cube_old::cube::Color::RED      => TOMATO,
//                 cube_old::cube::Color::BLUE     => CORNFLOWERBLUE,
//                 cube_old::cube::Color::PURPLE   => HOTPINK,
//                 cube_old::cube::Color::WHITE    => WHITESMOKE,
//             })
//             .points(pts[0], pts[1], pts[2], pts[3]);
//     }

//     draw.to_frame(app, &frame).unwrap();
// }



fn main() {
    // let cube = cube::cube::Cube::new(cube::maths::Vector3{x:0.,y:0.,z:0.}, 10.0);

    let world = cube::lib::World::new();
    println!("{:?}", world);
}