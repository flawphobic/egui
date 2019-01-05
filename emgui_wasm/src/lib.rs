#![deny(warnings)]

extern crate lazy_static;
extern crate serde_json;
extern crate wasm_bindgen;

extern crate emgui;

use std::sync::Mutex;

use emgui::{Emgui, Font, Frame, RawInput};

use wasm_bindgen::prelude::*;

mod app;
mod webgl;

fn font() -> Font {
    Font::new(20) // TODO: don't create this multiple times
}

#[wasm_bindgen]
pub fn new_webgl_painter(canvas_id: &str) -> Result<webgl::Painter, JsValue> {
    let emgui_painter = emgui::Painter::new(font()); // TODO: don't create this twice
    webgl::Painter::new(canvas_id, emgui_painter.texture())
}

struct State {
    app: app::App,
    emgui: Emgui,
    emgui_painter: emgui::Painter,
}

impl State {
    fn new() -> State {
        State {
            app: Default::default(),
            emgui: Emgui::new(font()),
            emgui_painter: emgui::Painter::new(font()),
        }
    }

    fn frame(&mut self, raw_input: RawInput) -> Frame {
        self.emgui.new_frame(raw_input);

        use crate::app::GuiSettings;
        self.app.show_gui(&mut self.emgui.layout);

        let mut style = self.emgui.style.clone();
        self.emgui.layout.foldable("Style", |gui| {
            style.show_gui(gui);
        });
        self.emgui.style = style;

        let commands = self.emgui.paint();
        self.emgui_painter.paint(&commands)
    }
}

#[wasm_bindgen]
pub fn paint_webgl(webgl_painter: &webgl::Painter, raw_input_json: &str) -> Result<(), JsValue> {
    // TODO: nicer interface than JSON
    let raw_input: RawInput = serde_json::from_str(raw_input_json).unwrap();

    lazy_static::lazy_static! {
        static ref STATE: Mutex<Option<State>> = Default::default();
    }

    let mut state = STATE.lock().unwrap();
    if state.is_none() {
        *state = Some(State::new());
    }
    let frame = state.as_mut().unwrap().frame(raw_input);
    webgl_painter.paint(&frame)
}