mod sort;
mod vecinit;
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
use vecviz::VecViz;
use webtools::{
    request_animation_frame,
    resize_canvas,
    get_element_by_id,
    get_button,
    get_playback_speed_input,
    get_playback_speed,
    window,
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

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let canvas_rc = Rc::new(RefCell::new(canvas));
    let context_rc = Rc::new(RefCell::new(context));
    let vv_rc = Rc::new(RefCell::new(VecViz::new(300, 300)));
    
    {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        let canvas = canvas_rc.borrow();
        let context = context_rc.borrow();
        resize_canvas(&canvas);
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        vv.init();
        vv.render(&canvas, &context);
    }

    start_animation_loop(vv_rc.clone(), canvas_rc.clone(), context_rc.clone());
    set_canvas_resize(canvas_rc.clone());
    set_interface_interactions(vv_rc);
    Ok(())
}

fn start_animation_loop(vv_rc: Rc<RefCell<VecViz>>, canvas_rc: Rc<RefCell<HtmlCanvasElement>>, context_rc: Rc<RefCell<CanvasRenderingContext2d>>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        let canvas = canvas_rc.borrow();
        let context = context_rc.borrow();
        if vv.running {
            vv.tick();
        }
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        vv.render(&canvas, &context);
        request_animation_frame(f.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn set_canvas_resize(canvas_rc: Rc<RefCell<HtmlCanvasElement>>) {
    let a = Closure::wrap(Box::new(move || {
        let canvas = canvas_rc.borrow();
        resize_canvas(&canvas);
    }) as Box<dyn FnMut()>);
    window().set_onresize(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_interface_interactions(vv_rc: Rc<RefCell<VecViz>>) {
    set_play_forwards(vv_rc.clone());
    set_play_backwards(vv_rc.clone());
    set_step_forwards(vv_rc.clone());
    set_step_backwards(vv_rc.clone());
    set_pause(vv_rc.clone());
    set_restart(vv_rc.clone());
    set_playback_speed(vv_rc.clone());
}

fn set_play_forwards(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.play_forwards();
    }) as Box<dyn FnMut()>);
    get_button("play-forwards").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_play_backwards(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.play_backwards();
    }) as Box<dyn FnMut()>);
    get_button("play-backwards").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_step_forwards(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.step_forwards();
    }) as Box<dyn FnMut()>);
    get_button("step-forwards").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_step_backwards(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.step_backwards();
    }) as Box<dyn FnMut()>);
    get_button("step-backwards").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}


fn set_pause(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.pause();
    }) as Box<dyn FnMut()>);
    get_button("pause").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
    
}

fn set_restart(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        vv.init();
    }) as Box<dyn FnMut()>);
    get_button("restart").set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

fn set_playback_speed(vv_rc: Rc<RefCell<VecViz>>) {
    let a = Closure::wrap(Box::new(move || {
        let mut vv = vv_rc.try_borrow_mut().unwrap();
        if let Ok(speed) = get_playback_speed() {
            vv.speed = speed
        }
    }) as Box<dyn FnMut()>);
    get_playback_speed_input().set_onchange(Some(a.as_ref().unchecked_ref()));
    a.forget();
}
