use rand::{
    Rng,
    seq::SliceRandom,
};

pub fn random(max: u32, len: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..len {
        vec.push(rng.gen_range(0, max))
    }
    vec
}

pub fn shuffled(max: u32, len: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut vec = ascending(max, len);
    vec.shuffle(&mut rng);
    vec
}

pub fn ascending(max: u32, len: u32) -> Vec<u32> {
    (0..len).map(|x| x*max/len).collect()
}

pub fn descending(max: u32, len: u32) -> Vec<u32> {
    (0..len).rev().map(|x| x*max/len).collect()
}
