use wasm_bindgen::prelude::*;
//use crate::Plottable;

trait Toggle {
    fn toggle_position(&mut self);
    fn toggle_velocity(&mut self);
}

impl Toggle for f32 {
    fn toggle_position(&mut self) {
        *self = 1.0;
    }

    fn toggle_velocity(&mut self) {
        *self = 0.0;
    }
}

#[wasm_bindgen]
pub struct WaveGridF32 {
    rows: usize,
    cols: usize,
    active_positions: Vec<f32>,
    active_velocities: Vec<f32>,
    auxiliary_positions: Vec<f32>,
    auxiliary_velocities: Vec<f32>
}

#[wasm_bindgen]
impl WaveGridF32 {

    pub fn new() -> WaveGridF32 {
        let rows = 64;
        let cols = 64;
        let length = rows * cols;
        WaveGridF32 {
            rows,
            cols,
            active_positions: vec![0.0; length],
            active_velocities: vec![0.0; length],
            auxiliary_positions: vec![0.0; length],
            auxiliary_velocities: vec![0.0; length]
        }
    }

}

impl WaveGridF32 {
    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn gradient(&self, center_index: isize, row: isize, col: isize) -> f32 {
        let cols_isize = self.cols as isize;
        let rows_isize = self.rows as isize;
        let center = self.active_positions[center_index as usize];
        let right = {
            if col + 1 < cols_isize {
                self.active_positions[(center_index + 1) as usize]
            } else {
                0.0
            }
        };
        let left = {
            if col > 0 {
                self.active_positions[(center_index - 1) as usize]
            } else {
                0.0
            }
        };
        let up = {
            if row + 1 < rows_isize {
                self.active_positions[(center_index + cols_isize) as usize]
            } else {
                0.0
            }
        };
        let down = {
            if row > 0 {
                self.active_positions[(center_index - cols_isize) as usize]
            } else {
                0.0
            }
        };
        right + left + up + down - 4.0 * center
    }

}

// Note, I was hoping to use the trait Plottable<f32> to ensure that the
// Rust implementation had all the functions javascript required, but,
// apparently due to namespace collision issues, it's not currently allowed.
// So instead this will be just a plain impl block rather than an
// impl Plottable<f32> for WaveGridF32 block.
#[wasm_bindgen]
impl /*Plottable<f32> for*/ WaveGridF32 {

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn update(&mut self) {
        let mut index: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let acceleration: f32 =
                    self.gradient(index as isize, row as isize, col as isize);
                let x = self.active_positions[index];
                let v = self.active_velocities[index];

                let dt: f32 = 0.0078125;
                let dx = v*dt + 0.5*acceleration*dt*dt;
                let dv = acceleration*dt;

                self.auxiliary_positions[index] = x + dx;
                self.auxiliary_velocities[index] = v + dv;
                
                index += 1;
            }
        }
        std::mem::swap(&mut self.active_positions,
                       &mut self.auxiliary_positions);
        std::mem::swap(&mut self.active_velocities,
                       &mut self.auxiliary_velocities);
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let index = self.get_index(row, col);
        self.active_positions[index].toggle_position();
        self.active_velocities[index].toggle_velocity();
    }

    pub fn data_pointer(&self) -> *const f32 {
        self.active_positions.as_ptr()
    }
}
