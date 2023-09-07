use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Error};

fn main() -> Result<(), Error> {
    let mut programs = HashMap::<String, String>::new();
    for file in fs::read_dir("/usr/share/applications/")? {
        let path = file?.path();
        let extension = path
            .extension()
            .ok_or_else(|| anyhow!("Extension could not be found"))?;
        if extension != "desktop" {
            continue;
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut name = String::new();
        let mut exec = String::new();
        for line in reader.lines() {
            let line = line?;
            if name.is_empty() && line.starts_with("Name=") {
                name = line.split('=').last().unwrap().to_string();
            } else if exec.is_empty() && line.starts_with("Exec=") {
                exec = line.split('=').last().unwrap().to_string();
            } else if !exec.is_empty() && !name.is_empty() {
                programs.insert(name, exec);
                break;
            }
        }
    }
    for program in programs {
        println!("{program:?}");
    }
    Ok(())
}
