
extern crate pid_sim;
extern crate pid;

use std::io;
use pid_sim::writer::OutputWriter;
use pid_sim::Simulation;
use pid::{PIDController, Controller};

struct PIDSimulation {
    pub sim: Simulation,
    pub pid: PIDController,
    pub writer: OutputWriter,
}

impl PIDSimulation{
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {
            sim: Simulation::new(),
            pid: PIDController::new(50, 1, -40, 100000),
            writer: OutputWriter::new()?,
        })
    }

    pub fn step(&mut self) -> Result<(), io::Error> {
        let res = self.pid.update((self.sim.temp() * 100.0) as i32, 1);
        self.sim.do_step(res as f32);
        self.writer.write(res as f32, self.sim.temp())?;
        Ok(())
    }
}

fn main() {
    let mut sim = PIDSimulation::new().unwrap();
    
    sim.pid.set_limits(0, 1024);
    sim.pid.set_target(7000); // set target of 70 degrees

    for  _ in 1..1000 {
        sim.step().unwrap()
    }

    sim.pid.set_target(10000);

    for _ in 1..1000 {
        sim.step().unwrap()
    }

    sim.pid.set_target(6000);

    for _ in 1..10000 {
        sim.step().unwrap()
    }
    //println!("Finished after {}!", t);
}

