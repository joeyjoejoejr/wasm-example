#[macro_use]
extern crate stdweb;

use kiss3d::event::WindowEvent;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use nalgebra::{UnitQuaternion, Vector3};

struct AppState {
  c: SceneNode,
  rot: UnitQuaternion<f32>,
}

impl State for AppState {
  fn step(&mut self, window: &mut Window) {
    self.c.prepend_to_local_rotation(&self.rot);

    for event in window.events().iter() {
      match event.value {
        WindowEvent::Scroll(delta_x, delta_y, _) => {
          console!(log, format!("Scroll: ({}, {})", delta_x, delta_y))
        }
        _ => {}
      }
    }
  }
}

fn main() {
  let mut window = Window::new("Kiss3d: wasm example");
  let mut c = window.add_cube(1.0, 1.0, 1.0);

  c.set_color(1.0, 0.0, 0.0);

  window.set_light(Light::StickToCamera);

  let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
  let state = AppState { c, rot };

  window.render_loop(state);
}
