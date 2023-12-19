use std::collections::HashMap;

pub fn subtract_from_all_keys(mut map: HashMap<usize, usize>, value: usize) -> HashMap<usize, usize> {
    for (_, v) in map.iter_mut() {
        *v -= value - 1;
    }
    map
}
