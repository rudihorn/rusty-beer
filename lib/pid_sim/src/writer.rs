use std::io;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct OutputWriter {
    pub file: File,
    pub timestep: u32,
}


impl OutputWriter {
    pub fn new() -> Result<Self, io::Error> {
        let p = Path::new("output.csv");
        let mut file = File::create(p)?;

        writeln!(file, "time,output,temp")?;

        Ok(OutputWriter {
            file: file,
            timestep: 0,
        })
    }

    pub fn write(&mut self, output: f32, temp: f32) -> Result<(), io::Error> {
        writeln!(self.file, "{},{},{}", self.timestep, output, temp)?;
        self.timestep = self.timestep + 1;
        Ok(())
    }

    pub fn close(self) {
        drop(self);
    }
}