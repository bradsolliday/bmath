use wasm_bindgen::prelude::*;

trait Toggle {
    fn toggle_position(&mut self);
    fn toggle_velocity(&mut self);
}

impl Toggle for i16 {
    fn toggle_position(&mut self) {
        *self = 108 * 128;
    }

    fn toggle_velocity(&mut self) {
        *self = 0;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(n: i32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i16(n: i16);
}

#[wasm_bindgen]
pub struct WaveGrid {
    rows: u32,
    cols: u32,
    active_positions: Vec<i16>,
    active_velocities: Vec<i16>,
    auxiliary_positions: Vec<i16>,
    auxiliary_velocities: Vec<i16>
}

#[wasm_bindgen]
impl WaveGrid {
    
    pub fn new() -> WaveGrid {
        let rows = 32;
        let cols = 32;
        let length = (rows * cols) as usize;
        WaveGrid {
            rows,
            cols,
            active_positions: vec![100 * 128; length],
            active_velocities: vec![0; length],
            auxiliary_positions: vec![0; length],
            auxiliary_velocities: vec![0; length]
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.cols + col) as usize
    }

    fn gradient(&self, center_index: i32, row: i32, col: i32) -> i16 {
        let cols_i32 = self.cols as i32;
        let rows_i32 = self.rows as i32;
        let right_index =
            if col + 1 < cols_i32 {
                center_index + 1
            } else {
                center_index
            };
        let left_index =
            if col > 0 {// should be > 0
                center_index - 1
            } else {
                center_index
            };
        let up_index =
            if row + 1 < rows_i32 {
                center_index + cols_i32
            } else {
                center_index
            };
        let down_index =
            if row > 0 {
                center_index - cols_i32 
            } else {
                center_index
            };
        let center = self.active_positions[center_index as usize] >> 1;
        let right  = self.active_positions[right_index as usize] >> 3;
        let left   = self.active_positions[left_index as usize] >> 3;
        let up     = self.active_positions[up_index as usize] >> 3;
        let down   = self.active_positions[down_index as usize] >> 3;
        right + left + up + down - center
    }
    
    pub fn update(&mut self) {
        let mut index: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let acceleration: i16 = self.gradient(index as i32, row as i32, col as i32) >> 4;

                let x = self.active_positions[index];
                if row == self.rows >> 1 && col == self.cols >> 1 {
                    log_i16(x);
                }
                let v = self.active_velocities[index];

                // letting dt = 1
                let dt = 1;
                let dx = match v.checked_add(acceleration >> 1) {
                    Some(v) => v,
                    None => {
                        log_i16(666);
                        i16::MAX
                    }
                } >> (dt + dt);
                let dv = acceleration >> dt;

                self.auxiliary_positions[index]  = x.checked_add(dx).unwrap_or(i16::MAX);
                self.auxiliary_velocities[index] = v.checked_add(dv).unwrap_or(i16::MAX);

                index += 1;
            }
        }
        std::mem::swap(&mut self.active_positions, &mut self.auxiliary_positions);
        std::mem::swap(&mut self.active_velocities, &mut self.auxiliary_velocities);
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }

    pub fn positions(&self) -> *const i16 {
        self.active_positions.as_ptr()
    }

    pub fn velocities(&self) -> *const i16 {
        self.active_velocities.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        self.active_positions[index].toggle_position();
        self.active_velocities[index].toggle_velocity();
    }
}

