
use std::f32;

pub mod writer;

pub const ROOM_TEMPERATURE : f32 = 20.00;
pub const TEMP_INCR_COEFF : f32 = 0.08269;
pub const TEMP_LOSS_COEFF : f32 = -0.008124;
pub const LEN : usize = 45;

pub struct Simulation {
    max: f32,
    min: f32,
    steps: [f32; LEN],
    mults: [f32; LEN],
    temp: f32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation{
            max: 255.0,
            min: 0.0,
            steps: [0.0; LEN],
            mults: [1.0/(LEN as f32); LEN],
            temp: ROOM_TEMPERATURE 
        }
    }

    pub fn do_step(&mut self, p: f32) {
        let mut add_temp = 0.0;

        let p = f32::min(self.max, f32::max(self.min, p));
        let p = TEMP_INCR_COEFF * p / 255.0;

        for i in 0..LEN {
            add_temp += self.mults[i] * self.steps[i]
        }

        for i in 1..LEN {
            self.steps[LEN - i] = self.steps[LEN - i - 1];
        }
        self.steps[0] = p;
        
        let loss = (self.temp - ROOM_TEMPERATURE) / 80.0 * TEMP_LOSS_COEFF;

        self.temp += add_temp + loss;
    }

    pub fn temp(&self) -> f32 {
        self.temp
    }
}
