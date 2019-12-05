use std::fs;

use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum OpCode {
    Add,
    Mut,
    End,
    Unknown,
}

impl OpCode {
    fn from_usize(v: usize) -> OpCode {
        match v {
            1 => OpCode::Add,
            2 => OpCode::Mut,
            99 => OpCode::End,
            _ => OpCode::Unknown,
        }
    }
}

fn load_input(path: PathBuf) -> Result<Vec<usize>, String> {
    let input_raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let input_raw_split: Vec<&str> = input_raw.split_terminator(',').collect();

    input_raw_split
        .into_iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>())
        .map(|m| m.map_err(|e| e.to_string()))
        .collect::<Result<Vec<usize>, String>>()
}

fn calc_next_state(
    input_state: Vec<usize>,
    start_address: usize,
) -> Result<(Vec<usize>, bool), String> {
    let op_code_val = input_state
        .get(start_address)
        .ok_or_else(|| "op_code_val error".to_owned())?;
    let op_code = OpCode::from_usize(*op_code_val);

    match op_code {
        OpCode::End => return Ok((input_state, true)),
        OpCode::Unknown => return Err("Unknown opcode".to_owned()),
        _ => (),
    }

    let mut next_state = input_state.clone();

    let pos1 = input_state
        .get(start_address + 1)
        .ok_or_else(|| "pos 1 error".to_owned())?;
    let pos2 = input_state
        .get(start_address + 2)
        .ok_or_else(|| "pos 2 error".to_owned())?;

    let target_pos = input_state
        .get(start_address + 3)
        .ok_or_else(|| "target_pos error".to_owned())?;

    let val1 = input_state
        .get(*pos1)
        .ok_or_else(|| "val 1 error".to_owned())?;
    let val2 = input_state
        .get(*pos2)
        .ok_or_else(|| "val 2 error".to_owned())?;

    let new_value = match op_code {
        OpCode::Add => val1 + val2,
        OpCode::Mut => val1 * val2,
        _ => panic!("should not be reached"),
    };

    next_state
        .get_mut(*target_pos)
        .map(|v| *v = new_value)
        .ok_or_else(|| "target_pos error".to_owned())?;

    Ok((next_state, false))
}

fn run_program(input: Vec<usize>) -> Result<Vec<usize>, String> {
    let l = input.len();
    let mut output: Vec<usize> = input;

    for x in (0..l).step_by(4) {
        let (output_next, done) = calc_next_state(output, x)?;
        output = output_next;

        if done {
            return Ok(output);
        }
    }

    Err("Program halted without opcode 99".to_owned())
}

fn find_noun_and_verb(input: &[usize], needle: usize) -> Result<(usize, usize), String> {
    for noun in 0..100 {
        for verb in 0..100 {
            let input_try = adjust_input(input, noun, verb)?;

            let output = run_program(input_try)?;
            let val_pos0 = get_pos(&output, 0)?;

            if val_pos0 == needle {
                return Ok((noun, verb));
            }
        }
    }

    Err("needle not found".to_owned())
}

fn get_pos(program: &[usize], pos: usize) -> Result<usize, String> {
    let val = program
        .get(pos)
        .ok_or_else(|| "program val 0 error".to_owned())?;

    Ok(*val)
}

fn adjust_input(input: &[usize], noun: usize, verb: usize) -> Result<Vec<usize>, String> {
    let mut input_adjusted = input.to_owned();

    input_adjusted
        .get_mut(1)
        .map(|v| *v = noun)
        .ok_or_else(|| "error adjusting noun (pos 1)".to_owned())?;

    input_adjusted
        .get_mut(2)
        .map(|v| *v = verb)
        .ok_or_else(|| "error adjusting verb (pos 2)".to_owned())?;

    Ok(input_adjusted)
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let input = load_input(PathBuf::from(INPUT_PATH))?;

    let input_1 = adjust_input(&input, 12, 2)?;

    let output = run_program(input_1)?;

    let val_pos0 = get_pos(&output, 0)?;

    println!("Output pos0: {}", val_pos0);

    let (noun, verb) = find_noun_and_verb(&input, 19_690_720)?;

    println!(
        "Noun: {}\tVerb: {}\tNeedle: {}",
        noun,
        verb,
        noun * 100 + verb
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_op_code_test_add_correct_output() {
        let t: &[usize] = &[1, 0, 0, 0, 99];
        assert_eq!(OpCode::from_usize(t[0]), OpCode::Add);
    }

    #[test]
    fn load_input_test_load_correct_output() {
        let input = load_input(PathBuf::from(INPUT_PATH));
        assert!(input.is_ok());
        let i = input.unwrap();
        assert_eq!(i.len(), 129);

        assert_eq!(
            i,
            vec!(
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 1, 13,
                23, 27, 1, 6, 27, 31, 1, 31, 10, 35, 1, 35, 6, 39, 1, 39, 13, 43, 2, 10, 43, 47, 1,
                47, 6, 51, 2, 6, 51, 55, 1, 5, 55, 59, 2, 13, 59, 63, 2, 63, 9, 67, 1, 5, 67, 71,
                2, 13, 71, 75, 1, 75, 5, 79, 1, 10, 79, 83, 2, 6, 83, 87, 2, 13, 87, 91, 1, 9, 91,
                95, 1, 9, 95, 99, 2, 99, 9, 103, 1, 5, 103, 107, 2, 9, 107, 111, 1, 5, 111, 115, 1,
                115, 2, 119, 1, 9, 119, 0, 99, 2, 0, 14, 0
            )
        )
    }

    #[test]
    fn run_test_single_input() {
        let test_data: Vec<usize> = vec![1, 0, 0, 0];
        let required_result: Vec<usize> = vec![2, 0, 0, 0];

        let test = calc_next_state(test_data, 0);

        assert!(test.is_ok());

        assert_eq!(test.unwrap().0, required_result);
    }

    #[test]
    fn run_test_program_1() {
        let test_data: Vec<usize> = vec![1, 0, 0, 0, 99];
        let required_result: Vec<usize> = vec![2, 0, 0, 0, 99];

        let test = run_program(test_data);

        assert!(test.is_ok());

        assert_eq!(test.unwrap(), required_result);
    }

    #[test]
    fn run_test_program_2() {
        let test_data: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let required_result: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let test = run_program(test_data);

        assert!(test.is_ok());

        assert_eq!(test.unwrap(), required_result);
    }

    #[test]
    fn run_test_program_3() {
        let test_data: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        let required_result: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];

        let test = run_program(test_data);

        assert!(test.is_ok());

        assert_eq!(test.unwrap(), required_result);
    }

    #[test]
    fn run_test_program_4() {
        let test_data: Vec<usize> = vec![2, 3, 0, 3, 99];
        let required_result: Vec<usize> = vec![2, 3, 0, 6, 99];

        let test = run_program(test_data);

        assert!(test.is_ok());

        assert_eq!(test.unwrap(), required_result);
    }
}
