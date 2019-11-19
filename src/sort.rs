pub fn insertion_sort(vec: &mut Vec<u32>) -> Vec<(usize, usize)> {
    let mut history = Vec::new();
    let mut j;
    for i in 1..vec.len() {
        j = i;
        while j > 0 && vec[j-1] > vec[j] {
            vec.swap(j-1, j);
            history.push((j-1, j));
            j -= 1;
        }
    }
    history
}

pub fn selection_sort(vec: &mut Vec<u32>) -> Vec<(usize, usize)> {
    let mut history = Vec::new();
    for i in 0..vec.len() {
        let mut min_j = i;
        for j in i..vec.len() {
            if vec[j] < vec[min_j] {
                min_j = j
            }
        }

        if min_j != i {
            history.push((i, min_j));
            vec.swap(i, min_j);
        }
    }
    history
}

pub fn quicksort(vec: &mut Vec<u32>) -> Vec<(usize, usize)> {
    let mut history = Vec::new();
    
    fn partition(vec: &mut Vec<u32>, lo: usize, hi: usize, history: &mut Vec<(usize, usize)>) -> usize {
        let pivot = vec[lo + (hi - lo)/2];
        let mut i: isize = (lo as isize) - 1;
        let mut j: isize = (hi as isize) + 1;
        loop {
            i += 1;
            while vec[i as usize] < pivot {
                i += 1;
            }
            j -= 1;
            while vec[j as usize] > pivot {
                j -= 1;
            }
            if i >= j {
                return j as usize
            }
            vec.swap(i as usize, j as usize);
            history.push((i as usize, j as usize));
        }
    }

    fn quicksort_(vec: &mut Vec<u32>, lo: usize, hi: usize, history: &mut Vec<(usize, usize)>) {
        if lo < hi {
            let p = partition(vec, lo, hi, history);
            quicksort_(vec, lo, p, history);
            quicksort_(vec, p+1, hi, history);
        }
    }

    quicksort_(vec, 0, vec.len()-1, &mut history);
    history
}
