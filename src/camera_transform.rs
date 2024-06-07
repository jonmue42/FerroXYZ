pub mod camera_transform {
    use macroquad::{camera::Camera3D, input::{is_key_down, KeyCode}, math::vec3, prelude::{Vec3, get_frame_time}};

    pub fn transform_camera(camera:&mut Camera3D, step_size: f32) {
        println!("camera pos: {}", camera.position);
        let xold: f32 = camera.position.x;
        let yold: f32  = camera.position.y;
        let zold: f32 = camera.position.z;
        
        let x: f32 = camera.position.x;
        let y: f32  = camera.position.y;
        let z: f32 = camera.position.z;

        let oldpos = camera.position;

        let mut sphere = cartesian_to_spherical(oldpos);
        println!("spherical coords: {}", sphere);

        if is_key_down(KeyCode::W) {
            sphere.x += step_size * get_frame_time();
        } else if is_key_down(KeyCode::S) {
            sphere.x -= step_size * get_frame_time();
        }

        if is_key_down(KeyCode::Q) {
            sphere.y += step_size * get_frame_time();
        } else if is_key_down(KeyCode::E) {
            sphere.y -= step_size * get_frame_time();
        }

        if is_key_down(KeyCode::A) {
            sphere.z += step_size * get_frame_time();
        } else if is_key_down(KeyCode::D) {
            sphere.z -= step_size * get_frame_time();
            
        }



        camera.position = spherical_to_cartesian(sphere);


       //camera.position = vec3(x, y, z)
    }

    fn cartesian_to_spherical(coords: Vec3) -> Vec3 {
        let radius = (coords.x.powi(2) + coords.y.powi(2) + coords.z.powi(2)).sqrt();
        let polar = if radius == 0.0  {
            println!("radius == 0.0 POLAR");
            0.0
        } else {
            (coords.z / radius).acos()
        };
        let azimuth = if coords.x == 0.0 && coords.y == 0.0 {
            println!("coords.x == 0.0 && coords.y == 0.0 AZIMUTH");
            0.0
        } else {
            coords.y.signum() * (coords.x / (coords.x.powi(2) + coords.y.powi(2)).sqrt()).acos()
        };
        vec3(radius, polar, azimuth)
    }

    fn spherical_to_cartesian(coords: Vec3) -> Vec3 {
        let x = coords.x * coords.y.sin() * coords.z.cos();
        let y = coords.x * coords.y.sin() * coords.z.sin();
        let z = coords.x * coords.y.cos();
        vec3(x, y, z)
    }

}
