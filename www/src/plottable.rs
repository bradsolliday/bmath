
pub trait Plottable<T> {

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn update(&mut self);
    fn toggle_cell(&mut self, row: usize, col: usize);
    
    fn data_pointer(&self) -> *const T;

}

