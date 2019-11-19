use js_sys::{
    Math,
};
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};
use crate::webtools::{
    get_algorithm,
    get_initialization,
    get_number_of_elements,
};
use crate::sort;
use crate::vecinit;

pub struct VecViz {
    start_vec: Vec<u32>,
    current_vec: Vec<u32>,
    current_step: usize,
    end_vec: Vec<u32>,
    history: Vec<(usize, usize)>,
    max: u32,
    len: u32,
    pub running: bool,
    pub done: bool,
}

impl VecViz {
    pub fn new(max: u32, len: u32) -> VecViz {
        VecViz {
            start_vec: Vec::new(),
            current_vec: Vec::new(),
            current_step: 0,
            end_vec: Vec::new(),
            history: Vec::new(),
            max: max,
            len: len,
            running: true,
            done: false,
        }
    }

    pub fn init(&mut self) {
        self.start_vec.clear();
        self.len = get_number_of_elements();
        self.max = self.len;
        self.start_vec = match get_initialization().as_ref() {
            "random" => vecinit::random,
            "shuffled" => vecinit::shuffled,
            "ascending" => vecinit::ascending,
            "descending" => vecinit::descending,
            _ => vecinit::shuffled,
        }(self.max, self.len);
        self.clear_history();
        self.apply_sort(match get_algorithm().as_ref() {
            "insertion sort" => sort::insertion_sort,
            "selection sort" => sort::selection_sort,
            "quicksort" => sort::quicksort,
            "bubble sort" => sort::bubble_sort,
            _ => sort::insertion_sort,
        });
        self.current_step = 0;
        self.done = false;
        self.running = true;
    }

    pub fn clear_history(&mut self) {
        self.current_vec = self.start_vec.clone();
        self.end_vec = self.start_vec.clone();
        self.history = Vec::new();
    }

    pub fn apply_sort<F>(&mut self, algorithm: F)
    where F: Fn(&mut Vec<u32>) -> Vec<(usize, usize)> {
        self.history = algorithm(&mut self.end_vec);
    }

    pub fn tick(&mut self) {
        if self.current_step == self.history.len() {
            self.done = true;
            self.running = false;
            return;
        }
        let (i, j) = self.history[self.current_step];
        self.current_vec.swap(i,j);
        self.current_step += 1;
    }

    pub fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
        let w: f64 = (canvas.width() as f64) / (self.current_vec.len() as f64);
        let max_h: f64 = canvas.height() as f64;
        for (i, n) in self.current_vec.iter().enumerate() {
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }
    }

    pub fn playpause(&mut self) {
        self.running = !self.running;
    }
}
