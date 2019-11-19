use std::{
    convert::TryInto,  
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Window,
    Document,
    Element,
    HtmlCanvasElement,
    HtmlSelectElement,
    HtmlButtonElement,
};

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

pub fn resize_canvas(canvas: &HtmlCanvasElement) -> () {
    canvas.set_width(canvas.client_width().try_into().unwrap());
    canvas.set_height(canvas.client_height().try_into().unwrap());    
}

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

pub fn get_algorithm() -> String {
    get_element_by_id("algorithm")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .map_err(|_| ())
        .unwrap()
        .value()
}

pub fn get_restart_button() -> HtmlButtonElement {
    get_element_by_id("restart")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn get_playpause_button() -> HtmlButtonElement {
    get_element_by_id("playpause")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap()
}