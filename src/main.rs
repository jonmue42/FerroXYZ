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


use bevy::{prelude::*};
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
    let path = Path::new("xyz_file.xyz");
    let atoms = read_file(&path);

    //draw atoms
    for atom in atoms.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.2).mesh()),
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
        distance.iter()
            .for_each(|distance| if distance.length() < 2.0 {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cylinder::new(0.05, distance.length())),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(atoms[indx].x.expect("REASON") + distance.x/2.0, atoms[indx].y.expect("REASON") + distance.y/2.0, atoms[indx].z.expect("REASON") + distance.z/2.0)
                        .with_rotation(Quat::from_rotation_arc(Vec3::new(0.0, 1.0, 0.0), distance.normalize())),
                    ..default()
                });
            
            })
    }

    //camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        ..default()
    },
    PanOrbitCamera::default(),
    ));
}
