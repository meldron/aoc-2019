use std::fs;
use std::path::PathBuf;
use std::convert::TryInto;

static BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

fn load_input(path: &PathBuf) -> Result<Vec<u8>, String> {
    let raw = fs::read_to_string(path).map_err(|e| {
        format!(
            "Could not read input data '{}': {}",
            path.as_path().display(),
            e
        )
    })?;

    raw.trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| format!("Could not parse '{}' as digit", c))
                .map(|u| u as u8)
        })
        .collect::<Result<Vec<u8>, String>>()
}

fn calc_next_phase(input: &[u8]) -> Vec<u8> {
    let mut output = input.to_owned();

    for (i, o) in output.iter_mut().enumerate().take(input.len()) {
        let mut val = 0i32;
        for (mut j, d) in input.iter().skip(i).enumerate() {
            j += i;
            let idx = (((j + 1) as i32 / (i + 1) as i32) % 4) as usize;
            val += (*d as i32 * BASE_PATTERN[idx] as i32) as i32;
        }
        *o = (val.abs() % 10) as u8;
    }

    output
}

fn message_start(message: &[u8], n: usize) -> Result<String, String> {
    message
        .iter()
        .take(n)
        .map(|i| {
            std::char::from_digit(*i as u32, 10)
                .ok_or_else(|| format!("Could not parse '{}' as char", i))
        })
        .collect::<Result<String, String>>()
}

fn iterate_phases(i: &mut Vec<u8>, n: usize) {
    for _ in 0..n {
        *i = calc_next_phase(i);
    }
}

fn digits_to_number(i: &[u8], n: usize) -> u32 {
    i.iter()
        .take(n)
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, x)| {
            (acc + (*x as u32) * 10u32.pow(i as u32))
        })
}

fn part2(input: &[u8], n: usize) -> Vec<u8> {
    let mut output = input.to_owned();

    for _ in 0..n {
        for i in (0..input.len()-1).rev() {
            output[i] = (output[i] + output[i + 1]) % 10;
        }
    }

    output
}

fn main() -> Result<(), String> {
    let input = load_input(&PathBuf::from("input/input.txt"))?;

    let mut output = input.clone();
    iterate_phases(&mut output, 100);
    let sol = message_start(&output, 8)?;
    println!("{}", sol);

    let offset = digits_to_number(&input, 7);

    let input2: Vec<u8> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(offset.try_into().unwrap())
        .copied()
        .collect();

    let output2 = part2(&input2, 100);

    let sol2 = message_start(&output2, 8)?;
    println!("{}", sol2);

    Ok(())
}
