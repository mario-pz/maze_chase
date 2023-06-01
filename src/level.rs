use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub struct Level {
    data: Vec<String>,
}

impl Level {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut data = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                data.push(line);
            }
        }

        Ok(Level { data })
    }

    pub fn get_data(&self) -> &[String] {
        &self.data
    }
}
