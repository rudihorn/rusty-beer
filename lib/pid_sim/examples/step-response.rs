extern crate pid_sim;

use pid_sim::Simulation;

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
