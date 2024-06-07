mod hello;
mod camera_transform;
mod xyz_parser;
use std::vec;
use std::{f32::consts::PI, path::Path};

use macroquad::ui::{root_ui, widgets};
pub use xyz_parser::read_file;
pub use xyz_parser::Atom;
pub use hello::print_hello;
pub use camera_transform::camera_transform::transform_camera;

use macroquad::prelude::*;

#[macroquad::main("FerroXYZ")]
async fn main() {
    let path = Path::new("xyz_file.xyz");
    //let path = Path::new("H2O.xyz");
    let mut position = vec3(5.0, 0.1, 0.1);
    let test_arr = vec![0, 1, 2, 3, 4, 5];
    
    let atoms = read_file(&path);
    let mut cam = Camera3D {
        position: position,
        up: vec3(0.0, 0.0, 1.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };
 
    loop {
        clear_background(LIGHTGRAY);
    
        widgets::Window::new(1, vec2(10.0, 10.0), vec2(100.0, 100.0))
            .label("Test")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {});

        println!("##################################");
        transform_camera(&mut cam, 1.0);
        println!("camera pos: {}", cam.position);

        set_camera(&cam);


 
        draw_grid(20, 1.0, RED, BLUE);
        for atom in atoms.iter() {
            draw_sphere(vec3(atom.x.expect("REASON") - atoms[0].x.expect("Reason"), atom.y.expect("REASON") - atoms[0].y.expect("Reason"), atom.z.expect("REASON") - atoms[0].z.expect("Reason")), 0.25, None, atom.color);
        }

        //calculated distance vectors
     //   let distances = atoms.iter()
     //       .enumerate()
     //       .flat_map(|(indx, atom)| atom.distances(&atoms[indx+1..]))
     //       .collect::<Vec<Vec3>>();
     // 
        let distances = atoms.iter()
            .enumerate()
            .map(|(indx, atom)| atom.distances(&atoms[indx+1..]))
            .collect::<Vec<Vec<Vec3>>>();


        for (indx, distance) in distances.iter().enumerate() {
            let center_shift = vec3(atoms[0].x.expect(""), atoms[0].y.expect(""), atoms[0].z.expect(""));
            //draw_cube(vec3(0.0, 0.0, 0.0), vec3(1.0, 10.0, 1.0), None, BLACK)
            draw_line_3d(vec3(10.0, 5.0, 0.0), vec3(0.0, 0.0, 0.0), GREEN);
            distance
                .iter()
                .for_each(|distance| if distance.length() < 2.0 {draw_line_3d(
                    vec3(atoms[indx].x.expect(""), atoms[indx].y.expect(""), atoms[indx].z.expect("")) - center_shift,
                    vec3(atoms[indx].x.expect(""), atoms[indx].y.expect(""), atoms[indx].z.expect("")) - center_shift + distance.to_owned(),
                    RED);
                    
                    println!("distance: {:?}", distance);
                    println!("abs::: {:?}", distance.length());}
                );

           // println!("Distance {:?}", distance);
           // println!("index {:?}", indx);
        }
 

        let distance = atoms[0].distances(&atoms);
        println!("Distances: {:?}", distances);

        //println!("Test arr: {:?}", &test_arr[1..4]);
        //println!("Test arr: {:?}", test_arr);

        set_default_camera();

        next_frame().await
    }

}
