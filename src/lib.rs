use std::{
    f64,
    cell::RefCell,
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};
use js_sys::{Math, JsString};
use std::convert::TryInto;
mod sort;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn resize_canvas(canvas: &HtmlCanvasElement) -> () {
    canvas.set_width(canvas.client_width().try_into().unwrap());
    canvas.set_height(canvas.client_height().try_into().unwrap());    
}

struct VecViz {
    vec: Vec<u32>,
    max: u32,
}

impl VecViz {
    fn new(max: u32) -> VecViz {
        VecViz {
            vec: Vec::new(),
            max: max,
        }
    }

    fn init_random(&mut self, n: usize) {
        self.vec.clear();
        for _ in 0..n {
            self.vec.push((Math::random()*(self.max as f64)) as u32);
        }
    }

    fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
        let w: f64 = (canvas.width() as f64) / (self.vec.len() as f64);
        let max_h: f64 = canvas.height() as f64;
        for (i, n) in self.vec.iter().enumerate() {
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    resize_canvas(&canvas);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&JsString::from("black"));
    let mut vv = VecViz::new(300);
    vv.init_random(300);
    vv.render(&canvas, &context);

    let mut is = sort::InsertionSort::new();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        is.tick(&mut vv.vec);
        vv.render(&canvas, &context);            
        if !is.done() {
            request_animation_frame(f.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
