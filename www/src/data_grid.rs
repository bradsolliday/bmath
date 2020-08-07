
use wasm_bindgen::prelude::*;
use std::num::Wrapping;

#[wasm_bindgen]
pub struct MyGrid {
    rows: u32,
    cols: u32,
    cells: Vec<Wrapping<u8>>
}

#[wasm_bindgen]
impl MyGrid {

    pub fn new() -> MyGrid {
        let rows = 16;
        let cols = 16;

        let cells = (0..rows * cols).map(|x| Wrapping(x as u8))
                                    .collect::<Vec<Wrapping<u8>>>();
        MyGrid {
            rows,
            cols,
            cells
        }
    }

    pub fn update(&mut self) {
        for p in &mut self.cells {
            *p += Wrapping(1);
        }
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }

    pub fn cells(&self) -> *const Wrapping<u8> {
        self.cells.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_MyGrid() {
        let mut mg = MyGrid::new();
        let ptr = mg.cells();
        for i in 0..256 {
            unsafe {
                assert_eq!(*ptr.offset(i), Wrapping(i as u8));
            }
        }
        mg.update();
        let ptr = mg.cells();
        unsafe {
            assert_eq!(*ptr, Wrapping(1));
            assert_eq!(*ptr.offset(20), Wrapping(21));
            assert_eq!(*ptr.offset(255), Wrapping(0));
        }
        mg.update();
        mg.update();
        let ptr = mg.cells();
        unsafe {
            assert_eq!(*ptr, Wrapping(3));
            assert_eq!(*ptr.offset(254), Wrapping(1));
        }
    }
}
