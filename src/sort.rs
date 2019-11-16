pub struct InsertionSort {
    i: u32,
    j: u32,
    done: bool,
}

impl InsertionSort {
    pub fn new() -> InsertionSort {
        InsertionSort {
            i: 1,
            j: 1,
            done: false
        }
    }

    pub fn tick(&mut self, vec: &mut Vec<u32>) {
        if self.i < vec.len() as u32 {
            let j = self.j as usize;
            if self.j > 0 && vec[j-1] > vec[j] {
                vec.swap(j-1, j);
                self.j -= 1;
            } else {
                self.i += 1;
                self.j = self.i;
            }
        } else {
            self.done = true;
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }
}

