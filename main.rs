extern crate kiss3d;
extern crate nalgebra as na;
extern crate env_logger; 

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use na::{UnitQuaternion, Vector3, Translation3, Point3};
use rand::Rng;

fn generate_color_code (time:f32, rgb:(f32, f32, f32)) -> (f32, f32, f32) {
  let red: f32 = ((time*rgb.0*5.0).sin()+1.0)/2.0;
  let green: f32 = ((time*rgb.1*2.0).sin()+1.0)/2.0;
  let blue: f32 = ((time*rgb.2*3.0).sin()+1.0)/2.0;

  (red, green, blue)
}

fn main() {
    env_logger::init();
    let eye = Point3::new(0.6, 0.6, -2.0);
    let at = Point3::new(0.6, 0.6, 0.0);
    let mut arc_ball = ArcBall::new(eye, at);
    let mut window = Window::new("Kiss3d: cube");

    let mut cubes = vec![];
    let mut rgb_codes = vec![];

    let mut rng = rand::thread_rng();
    
    let mut position_x = 0.0;
    let mut position_y = 0.0;

    for i in 0..15 {
      position_x += 0.3;

      if i % 5 == 0 {
        position_x = 0.0;
        position_y += 0.3;
      };

      let mut cube = window.add_cube(0.2, 0.2, 0.2);
      let rgb: (f32, f32, f32) =  rng.gen();

      cube.set_local_translation(Translation3::new(position_x, position_y, 0.0));
      cubes.push(cube);
      rgb_codes.push(rgb);
    }


    window.set_light(Light::StickToCamera);
    


    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut time: f32 = 0.0;
    while window.render_with_camera(&mut arc_ball) {
      time += 0.05;

      for (i, cube) in cubes.iter_mut().enumerate() {
        let color = generate_color_code(time, rgb_codes[i]);
        cube.set_color(color.0, color.1, color.2);
        cube.prepend_to_local_rotation(&rot);
      };
    }
}
