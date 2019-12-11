use crate::vecviz::{
    History,
    HistoryElement,
};

use HistoryElement::*;

fn cmp(v: &Vec<u32>, h: &mut History, i: usize, j: usize) -> bool {
    h.push(Compare(i, j));
    v[i] > v[j]
}

pub fn insertion_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    let mut j;
    for i in 1..vec.len() {
        j = i;
        while j > 0 && cmp(&vec, &mut history, j-1, j) {
            vec.swap(j-1, j);
            history.push(Swap(j-1, j));
            j -= 1;

        }
    }
    history
}

pub fn selection_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    for i in 0..vec.len() {
        let mut min_j = i;
        for j in i..vec.len() {
            if cmp(&vec, &mut history, min_j, j) {
                min_j = j
            }
        }

        if min_j != i {
            history.push(Swap(i, min_j));
            vec.swap(i, min_j);
        }
    }
    history
}

pub fn gnome_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    let mut pos = 0;
    while pos < vec.len() {
        if pos == 0 || !cmp(&vec, &mut history, pos-1, pos) {
            pos += 1;
        } else {
            vec.swap(pos, pos-1);
            history.push(Swap(pos, pos-1));
            pos -= 1;
        }
    }
    history
}

pub fn cocktail_shaker_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..vec.len()-1 {
            if cmp(&vec, &mut history, i, i+1) {
                vec.swap(i, i+1);
                history.push(Swap(i, i+1));
                swapped = true;
            }
        }
        if !swapped {
            break
        }
        for i in (1..vec.len()-2).rev() {
            if cmp(&vec, &mut history, i, i+1) {
                vec.swap(i, i+1);
                history.push(Swap(i, i+1));
                swapped = true;
            }
        }
    }
    history
}

pub fn quicksort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    
    fn partition(vec: &mut Vec<u32>, lo: usize, hi: usize, mut history: &mut History) -> usize {
        let mut i = lo;
        let mut j = hi;
        let mut pivot = (lo + hi)/2;
        loop {
            while cmp(&vec, &mut history, pivot, i) {
                i += 1;
            }
            while cmp(&vec, &mut history, j, pivot) {
                j -= 1;
            }
            if i > j {
                return i;
            }
            if pivot == i {
                pivot = j;
            } else if pivot == j {
                pivot = i;
            }
            history.push(Swap(i, j));
            vec.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    }

    fn quicksort_(vec: &mut Vec<u32>, lo: usize, hi: usize, history: &mut History) {
        if lo < hi {
            let p = partition(vec, lo, hi, history);
            quicksort_(vec, lo, p-1, history);
            quicksort_(vec, p, hi, history);
        }
    }

    quicksort_(vec, 0, vec.len()-1, &mut history);
    history
}

pub fn bubble_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    for j in (0..vec.len()).rev() {
        for i in 0..j {
            if cmp(&vec, &mut history, i, i+1) {
                vec.swap(i, i+1);
                history.push(Swap(i, i+1));
            }
        }
    }
    history
}

pub fn shellsort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    let gaps = [701, 301, 132, 57, 23, 10, 4, 1];
    let n = vec.len();
    for &gap in gaps.iter() {
        for i in gap..n {
            let mut j = i;
            while j >= gap && cmp(&vec, &mut history, j-gap, j) {
                vec.swap(j-gap, j);
                history.push(Swap(j-gap, j));
                j -= gap;
            }
        }
    }
    history
}

pub fn comb_sort(vec: &mut Vec<u32>) -> History {
    let mut history = Vec::new();
    let mut gap = vec.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f64/shrink) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..vec.len()-gap {
            if cmp(&vec, &mut history, i, i+gap) {
                vec.swap(i, i+gap);
                history.push(Swap(i, i+gap));
                sorted = false;
            }
        }
    }
    history
}
