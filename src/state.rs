use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

lazy_static! {
    static ref STATE: Mutex<Arc<State>> = Mutex::new(Arc::new(State::new()));
}

pub fn get_curr_state() -> Arc<State> {
    STATE.lock().unwrap().clone()
}

pub struct State {
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_x_axis: f32,
    pub rotation_y_axis: f32,
}

impl State {
    fn new() -> Self {
        Self {
            mouse_down: false,
            mouse_x: -1.,
            mouse_y: -1.,
            rotation_x_axis: -65. * std::f32::consts::PI / 180., // left-hand rule
            rotation_y_axis: -0. * std::f32::consts::PI / 180.,
        }
    }
}

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = STATE.lock().unwrap();
    *data = Arc::new(State {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: y,
        ..*data.clone()
    });
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = STATE.lock().unwrap();
    let x_delta = x - data.mouse_x;
    let y_delta = y - data.mouse_y;
    let rotation_x_delta = if data.mouse_down {
        std::f32::consts::PI * y_delta / 600.
    } else {
        0.
    };
    let rotation_y_delta = if data.mouse_down {
        std::f32::consts::PI * x_delta / 600.
    } else {
        0.
    };

    *data = Arc::new(State {
        mouse_x: x,
        mouse_y: y,
        rotation_x_axis: data.rotation_x_axis + rotation_x_delta,
        rotation_y_axis: data.rotation_y_axis + rotation_y_delta,
        ..*data.clone()
    });
}

pub fn attach_hander() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let handler = move |event: web_sys::PointerEvent| {
        update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("pointerdown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    let handler = move |event: web_sys::PointerEvent| {
        update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("pointerup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    let handler = move |event: web_sys::PointerEvent| {
        update_mouse_position(event.client_x() as f32, event.client_y() as f32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("pointermove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
