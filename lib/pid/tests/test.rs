extern crate pid;
#[cfg(test)]
mod tests {
    use pid;
    use pid::Controller;
    
    #[test]
    fn works_in_100_steps() {
        let mut steps = 0;
        let mut controller = pid::PIDController::new(100, 0, 0);
        controller.set_target(120);
        let mut value = 0;
        while(steps < 100) {
            value = controller.update(value, 1);
            steps += 1;
        }
        assert_eq!(120, value);
    }
}