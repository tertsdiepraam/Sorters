use std::{
    f64,
    cell::RefCell,
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Window,
    Document,
    Element,
    HtmlElement,
    HtmlCanvasElement,
    HtmlSelectElement,
    HtmlButtonElement,
    CanvasRenderingContext2d,
    console,
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

fn window() -> Window {
    web_sys::window().unwrap()
}

fn document() -> Document {
    window().document().unwrap()
}

fn get_element_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

fn get_algorithm() -> String {
    get_element_by_id("algorithm")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .map_err(|_| ())
        .unwrap()
        .value()
}

fn get_restart_button() -> HtmlButtonElement {
    get_element_by_id("restart")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap()
}

struct VecViz {
    start_vec: Vec<u32>,
    current_vec: Vec<u32>,
    current_step: usize,
    end_vec: Vec<u32>,
    history: Vec<(usize, usize)>,
    max: u32,
    done: bool,
}

impl VecViz {
    fn new(max: u32) -> VecViz {
        VecViz {
            start_vec: Vec::new(),
            current_vec: Vec::new(),
            current_step: 0,
            end_vec: Vec::new(),
            history: Vec::new(),
            max: max,
            done: false,
        }
    }

    fn init_random(&mut self, n: usize) {
        self.start_vec.clear();
        for _ in 0..n {
            self.start_vec.push((Math::random()*(self.max as f64)) as u32);
        }
        self.clear_history();
    }

    fn clear_history(&mut self) {
        self.current_vec = self.start_vec.clone();
        self.end_vec = self.start_vec.clone();
        self.history = Vec::new();
    }

    fn apply_sort<F>(&mut self, algorithm: F)
    where F: Fn(&mut Vec<u32>) -> Vec<(usize, usize)> {
        self.history = algorithm(&mut self.end_vec);
    }

    fn tick(&mut self) {
        let (i, j) = self.history[self.current_step];
        self.current_vec.swap(i,j);
        self.current_step += 1;
        if self.current_step == self.history.len() {
            self.done = true;
        }
    }

    fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
        let w: f64 = (canvas.width() as f64) / (self.current_vec.len() as f64);
        let max_h: f64 = canvas.height() as f64;
        for (i, n) in self.current_vec.iter().enumerate() {
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }
    }

    fn restart(&mut self) {
        self.init_random(self.start_vec.len());
        self.apply_sort(match get_algorithm().as_ref() {
            "insertion sort" => sort::insertion_sort,
            "selection sort" => sort::selection_sort,
            "quicksort" => sort::quicksort,
            _ => sort::insertion_sort,
        });
        self.current_step = 0;
        self.done = false;
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    let canvas = get_element_by_id("canvas")
        .unwrap()
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

    let vv_rc = Rc::new(RefCell::new(VecViz::new(300)));

    {
        let vv_rc = vv_rc.clone();
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.init_random(300);
        vv.apply_sort(match get_algorithm().as_ref() {
            "insertion sort" => sort::insertion_sort,
            "selection sort" => sort::selection_sort,
            "quicksort" => sort::quicksort,
            _ => sort::insertion_sort,
        });
        vv.render(&canvas, &context);
    }
    {
        let vv_rc = vv_rc.clone();
        let a = Closure::wrap(Box::new(move || {
            let mut vv = vv_rc.try_borrow_mut().unwrap();
            vv.restart();
        }) as Box<dyn FnMut()>);
        get_restart_button().set_onclick(Some(a.as_ref().unchecked_ref()));
        a.forget();
    }
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let vv_rc = vv_rc.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut vv = vv_rc.try_borrow_mut().unwrap();
            if !vv.done {
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                vv.tick();
                vv.render(&canvas, &context);
            }
            request_animation_frame(f.borrow().as_ref().unwrap())
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
    Ok(())
}
