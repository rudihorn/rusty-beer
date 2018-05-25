use std::cmp;
use std::f32;

const room_temperature : f32 = 20.00;
const temp_incr_coeff : f32 = 0.08269;
const temp_loss_coeff : f32 = -0.008124;
const len : usize = 45;

struct Simulation {
    max: f32,
    min: f32,
    steps: [f32; len],
    mults: [f32; len],
    temp: f32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation{
            max: 255.0,
            min: 0.0,
            steps: [0.0; len],
            mults: [1.0/(len as f32); len],
            temp: room_temperature
        }
    }

    pub fn do_step(&mut self, p: f32) {
        let mut add_temp = 0.0;

        let p = f32::min(self.max, f32::max(self.min, p));
        let p = temp_incr_coeff * p / 255.0;

        for i in 0..len {
            add_temp += self.mults[i] * self.steps[i]
        }

        for i in 1..len {
            self.steps[len - i] = self.steps[len - i - 1];
        }
        self.steps[0] = p;
        
        let loss = (self.temp - room_temperature) / 80.0 * temp_loss_coeff;

        self.temp += add_temp + loss;
    }
}

fn main() {
    let mut sim = Simulation::new();

    println!("time p temp");

    let mut t = 0;
    loop {
        sim.do_step(255.0);
        println!("{} 1 {}", t, sim.temp);
        if sim.temp > 100.0 { break; }
        t = t + 1;
    }

    let runmore = t*2;

    for _ in 1..runmore {
        sim.do_step(0.0);
        println!("{} 0 {}", t, sim.temp);
        t = t + 1;
    }

    //println!("Finished after {}!", t);
}
