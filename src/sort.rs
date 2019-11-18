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
