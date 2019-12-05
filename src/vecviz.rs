use js_sys::{
    Math,
    JsString,
};
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};
use crate::webtools::{
    get_algorithm,
    get_initialization,
    get_number_of_elements,
    get_playback_speed,
};
use crate::sort;
use crate::vecinit;

#[derive(Clone, Copy)]
enum Direction {
    Backwards,
    Forwards,
}

pub enum HistoryElement {
    Swap(usize, usize),
    Compare(usize, usize),
}

use HistoryElement::*;

pub type History = Vec<HistoryElement>;

use Direction::*;

pub struct VecViz {
    start_vec: Vec<u32>,
    current_vec: Vec<u32>,
    current_step: usize,
    end_vec: Vec<u32>,
    history: History,
    highlighted_indices: (Option<usize>, Option<usize>),
    max: u32,
    len: u32,
    direction: Direction,
    pub speed: u32,
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
            highlighted_indices: (None, None),
            max: max,
            len: len,
            direction: Forwards,
            speed: 1,
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
            "gnome sort" => sort::gnome_sort,
            "cocktail shaker sort" => sort::cocktail_shaker_sort,
            _ => sort::insertion_sort,
        });
        self.current_step = 0;
        if let Ok(speed) = get_playback_speed() {
            self.speed = speed;
        }
        self.done = false;
        self.running = true;
    }

    fn clear_history(&mut self) {
        self.current_vec = self.start_vec.clone();
        self.end_vec = self.start_vec.clone();
        self.history = Vec::new();
    }

    fn apply_sort<F>(&mut self, algorithm: F)
    where F: Fn(&mut Vec<u32>) -> History {
        self.history = algorithm(&mut self.end_vec);
    }

    pub fn tick(&mut self) {
        for _ in 0..self.speed {
            self.step_in_dir(self.direction);
        }
    }

    pub fn step_forwards(&mut self) {
        self.step_in_dir(Forwards);
    }

    pub fn step_backwards(&mut self) {
        self.step_in_dir(Backwards);
    }

    fn step_in_dir(&mut self, dir: Direction) {
        let limit = match dir {
            Forwards => self.history.len(),
            Backwards => 0,
        };
        if self.current_step == limit {
            self.done = true;
            self.running = false;
            self.highlighted_indices = (None, None);
            return;
        }
        match dir {
            Forwards => {
                match self.history[self.current_step] {
                    Swap(i,j) => {
                        self.current_vec.swap(i,j);
                        self.current_step += 1;
                    }
                    Compare(i,j) => {
                        self.highlighted_indices = (Some(i), Some(j));
                        self.current_step += 1
                    }
                }
            }
            Backwards => {
                match self.history[self.current_step-1] {
                    Swap(i,j) => {
                        self.current_vec.swap(i,j);
                        self.current_step -= 1;
                    }
                    Compare(i,j) => {
                        self.highlighted_indices = (Some(i), Some(j));
                        self.current_step -= 1;
                    }
                }
            }
        }
    }

    pub fn render(&self, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
        let w: f64 = (canvas.width() as f64) / (self.current_vec.len() as f64);
        let max_h: f64 = canvas.height() as f64;
        context.set_fill_style(&JsString::from("black"));
        for (i, n) in self.current_vec.iter().enumerate() {
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }

        context.set_fill_style(&JsString::from("red"));
        let (a, b) = self.highlighted_indices;
        if let Some(i) = a {
            let n = self.current_vec[i];
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }
        if let Some(i) = b {
            let n = self.current_vec[i];
            let h: f64 = (n.clone() as f64) / (self.max as f64) * max_h;
            let x = Math::floor((i as f64)*w);
            context.fill_rect(x, max_h, Math::ceil(w), -h);
        }
    }

    pub fn play_forwards(&mut self) {
        self.running = true;
        self.direction = Forwards;
    }

    pub fn play_backwards(&mut self) {
        self.running = true;
        self.direction = Backwards;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }
}
