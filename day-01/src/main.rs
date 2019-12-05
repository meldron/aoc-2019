use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

static INPUT_PATH: &str = "input/input.txt";

fn load_masses(path: PathBuf) -> Result<Vec<u32>, String> {
    let f = File::open(path).map_err(|e| e.to_string())?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.map_err(|e| e.to_string()))
        .collect::<Result<Vec<String>, String>>()?;

    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u32>())
        .map(|m| m.map_err(|e| e.to_string()))
        .collect::<Result<Vec<u32>, String>>()
}

fn calc_fuel(mass: u32) -> u32 {
    0.max(((mass / 3) as i32) - 2) as u32
}

fn calc_fuel_rec(mass: u32) -> u32 {
    let mass = 0.max(((mass / 3) as i32) - 2) as u32;
    match mass {
        0 => 0,
        _ => calc_fuel_rec(mass) + mass,
    }
}

fn main() -> Result<(), String> {
    let masses = load_masses(PathBuf::from(INPUT_PATH))?;

    // part 1
    let fuel_required: Vec<u32> = masses.iter().map(|m| calc_fuel(*m)).collect();
    let fuel_sum: u32 = fuel_required.into_iter().sum();

    println!("Fuel sum: {}", fuel_sum);

    // part 2
    let fuel_required_rec: Vec<u32> = masses.into_iter().map(calc_fuel_rec).collect();
    let fuel_sum_rec: u32 = fuel_required_rec.into_iter().sum();

    println!("Fuel sum rec: {}", fuel_sum_rec);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_fuel_supplied_inputs_output_correct() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100_756), 33583);
    }

    #[test]
    fn load_masses_supplied_inputs_output_correct() {
        let masses = load_masses(PathBuf::from(INPUT_PATH));
        assert!(masses.is_ok());
        let m = masses.unwrap();
        assert_eq!(m.len(), 100);
    }

    #[test]
    fn load_masses_broken_input_output_error() {
        let masses = load_masses(PathBuf::from("input/input.broken.txt"));
        assert!(masses.is_err());
    }

    #[test]
    fn calc_fuel_rec_supplied_inputs_output_correct() {
        assert_eq!(calc_fuel_rec(12), 2);
        assert_eq!(calc_fuel_rec(1969), 966);
        assert_eq!(calc_fuel_rec(100_756), 50_346);
    }
}
