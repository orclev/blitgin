use crate::core::error::GameError;
use crate::core::input::Input;

#[cfg(feature = "console_error_panic_hook")]
use std::panic;

use wasm_bindgen::prelude::*;
use log::*;
use crate::asset::render_object::RenderObject;

pub mod asset;
pub mod core;

#[wasm_bindgen(start)]
pub fn init_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("error initializing log");
}

#[wasm_bindgen]
pub fn has_wait_for(key: &str) -> bool {
    debug!("has wait for called");
    let mut input: Input = Default::default();
    debug!("new input created");
    input.add_wait_for_action(String::from(key), 32);
    debug!("wait for action called");
    GameError::warn(&"some message");
    if input.has_waits_for(key) {
        debug!("in the if true");
        let mut render_obj = RenderObject {
            transparency: false,
            show_bounds: false,
            color_constant:String::from("someValue").into_boxed_str(),
            workbench: None,
            x: 0,
            y: 0,
            original_x: 0,
            original_y: 0,
            width: 0,
            height: 0,
            duration: 0,
            frame: 0,
            frame_buffer: 0,
            velocity_x: 0,
            velocity_y: 0,
            ease_coefficient: 0,
            index: 0,
            life_span: 0,
            cell_width: 0,
            cell_height: 0,
            direction: 0,
            asset_class: String::from("someValue").into_boxed_str(),
            keyframe_len: 0
        };
        let result = render_obj.init();
        if result.is_ok() {
            debug!("result is true");
        }
        return true
    }
    debug!("false");
    false
}
