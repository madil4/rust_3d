use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod animator;
mod body;
mod camera;
mod grid;
mod mesh;
mod state;
mod viewer;

#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let performance = web_sys::window()
        .unwrap()
        .performance()
        .expect("performance should be available");
    let dt = 1000.0 / 30.0;
    let mut last_draw_time = -1.;
    state::attach_hander().unwrap();
    let viewer = viewer::Viewer::new();
    let mut camera = camera::Camera::new();
    let grid = grid::Grid::new();
    let animator = animator::Animator::new();
    let mut body = body::Body::new(
        mesh::mesh_vertices::get(),
        mesh::mesh_tets::get(),
        mesh::obj_faces::get(),
    );

    // the loop
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = performance.now();
        if current_time >= last_draw_time + dt {
            last_draw_time = current_time;
            log("in the loop");
            let curr_state = state::get_curr_state();
            camera.set_rotations(curr_state.rotation_x_axis, curr_state.rotation_y_axis);
            // animator.aimate(&mut body);
            viewer.render(&camera, &grid, &body);
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
