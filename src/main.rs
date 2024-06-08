//mod hello;
//mod camera_transform;
mod xyz_parser;
use core::f32;
use std::{any::type_name_of_val, path::Path};

//use std::vec;
//use std::{f32::consts::PI, path::Path};
//
pub use xyz_parser::read_file;
//pub use xyz_parser::Atom;
//pub use hello::print_hello;
//pub use camera_transform::camera_transform::transform_camera;


use bevy::{prelude::*, reflect::List};
use bevy_panorbit_camera::{self, PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::FUCHSIA))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let path = Path::new("H2O.xyz");
    let atoms = read_file(&path);

    //draw atoms
    for atom in atoms.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh()),
            material: materials.add(atom.color),
            transform: Transform::from_xyz(atom.x.expect("Err"), atom.y.expect("Err"), atom.z.expect("Err")),
            ..default()
        });
    }

    //draw bonds
    let distances = atoms.iter()
        .enumerate()
        .map(|(indx, atom)| atom.distances(&atoms[indx+1..]))
        .collect::<Vec<Vec<Vec3>>>();

    for (indx, distance) in distances.iter().enumerate() {
        println!("Distance {:?}", distance);
        let distance_vec =<Vec<Vec3> as FromReflect>::from_reflect(distance).unwrap();
        for dis in distance_vec.iter() {
            let dis = <Vec3 as FromReflect>::from_reflect(dis).unwrap();
            if dis.length() < 2.0 {
                println!("dis < 2: {:?}", dis);
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cylinder::new(0.1, 5.0).mesh()),
                    material: materials.add(Color::WHITE),
                    transform: Transform { translation: Vec3::new(atoms[indx].x.expect("Err"), atoms[indx].y.expect("Err"), atoms[indx].z.expect("Err")), ..default() },
                    ..default()
                });
        

            }

        }
    }

       // distance_vec.iter()
       //    .for_each(|distance_vec| if distance_vec.length() < 2.0 {
       //        println!("distance: {:?}", distance_vec.length());
       //    })


    //camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        ..default()
    },
    PanOrbitCamera::default(),
    ));
}
