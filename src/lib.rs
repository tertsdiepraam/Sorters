mod sort;
mod vecviz;
mod webtools;
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
use js_sys::JsString;
use vecviz::VecViz;
use webtools::{
    request_animation_frame,
    resize_canvas,
    get_element_by_id,
    get_algorithm,
    get_restart_button,
    get_playpause_button,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let vv_rc = vv_rc.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut vv = vv_rc.try_borrow_mut().unwrap();
            if vv.running {
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                vv.tick();
                vv.render(&canvas, &context);
            }
            request_animation_frame(f.borrow().as_ref().unwrap())
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    set_interface_interactions(vv_rc);
    Ok(())
}

fn set_interface_interactions(vv_rc: Rc<RefCell<VecViz>>) {
    set_playpause_button(vv_rc.clone());
    set_restart_button(vv_rc.clone());
}

fn set_playpause_button(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.playpause();
    }) as Box<dyn FnMut()>);
    get_playpause_button().set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_restart_button(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.restart();
    }) as Box<dyn FnMut()>);
    get_restart_button().set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}
