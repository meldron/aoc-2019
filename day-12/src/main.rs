use regex::Regex;
use std::fs;
use std::path::PathBuf;

use unroll::unroll_for_loops;

fn load_input(path: &PathBuf) -> Result<Vec<[i64; 3]>, String> {
    let re = Regex::new(r"<x=(-?\d*?), y=(-?\d*?), z=(-?\d*?)>").map_err(|e| e.to_string())?;

    let input_raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let input: Vec<[i64; 3]> = input_raw
        .lines()
        .filter_map(|l| {
            let caps = re.captures(l)?;
            let x = caps.get(1)?.as_str().parse::<i64>();
            let y = caps.get(2)?.as_str().parse::<i64>();
            let z = caps.get(3)?.as_str().parse::<i64>();
            Some([
                x.expect("could not parse x"),
                y.expect("could not parse y"),
                z.expect("could not parse z"),
            ])
        })
        .collect();

    Ok(input)
}

static INPUT_PATH: &str = "input/input.txt";

use std::cmp::Ordering;
//use std::fs;

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut remainder;

    loop {
        remainder = a % b;
        a = b;
        b = remainder;

        if b == 0 {
            break a;
        }
    }
}

#[unroll_for_loops]
fn main() {
    let mut pos: [[i64; 4]; 3];
    let mut vel: [[i64; 4]; 3] = [[0; 4]; 3];

    // pos = [[7, 10, 17], [-2, 7, 0], [12, 5, 12], [5, -8, 6]];
    pos = [[7, -2, 12, 5], [10, 7, 5, -8], [17, 0, 12, 6]];

    // let data = fs::read_to_string("input12.txt").expect("Couldn't read input file.");
    // println!("{:?}", data);

    let pos0 = pos;
    let vel0 = vel;

    let mut periods = [0, 0, 0];

    // calculate periods for each dimension
    for dim in 0..3 {
        let mut i: u64 = 0;

        loop {
            // apply gravity to each moon
            for (moon1, p1) in pos[dim].iter().enumerate() {
                for moon2 in moon1 + 1..4 {
                    if *p1 < pos[dim][moon2] {
                        vel[dim][moon1] += 1;
                        vel[dim][moon2] -= 1;
                    } else if *p1 > pos[dim][moon2] {
                        vel[dim][moon1] -= 1;
                        vel[dim][moon2] += 1;
                    }
                }
            }

            // adjust velocity
            for moon in 0..4 {
                pos[dim][moon] += vel[dim][moon];
            }

            i += 1;

            if pos[dim] == pos0[dim] && vel[dim] == vel0[dim] {
                break;
            }
        }

        periods[dim] = i;
    }
    let result = periods.iter().fold(1, |acc, period| lcm(acc, *period));
    println!(
        "Periods are {:?}, same after {:?} iterations.",
        periods, result
    );
}

// fn main() -> Result<(), String> {
//     let input = load_input(&PathBuf::from(INPUT_PATH))?;
//     println!("{:?}", input);

//     let test = [[0; 4]; 3];
//     println!("{:?}", test);

//     Ok(())
// }
