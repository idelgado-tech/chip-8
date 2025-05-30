pub fn pos_from_index(index: usize,width : usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

pub fn index_from_pos(x: usize, y: usize,width : usize) -> usize {
    (y * width) + x
}