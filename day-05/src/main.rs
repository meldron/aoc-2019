// #![allow_dead_code]
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum OpCode {
    Add {
        p1_mode: Mode,
        p2_mode: Mode,
        p3_mode: Mode,
    },
    Mut {
        p1_mode: Mode,
        p2_mode: Mode,
        p3_mode: Mode,
    },
    Input,
    Output {
        p1_mode: Mode,
    },
    JumpIfTrue {
        p1_mode: Mode,
        p2_mode: Mode,
    },
    JumpIfFalse {
        p1_mode: Mode,
        p2_mode: Mode,
    },
    LessThan {
        p1_mode: Mode,
        p2_mode: Mode,
        p3_mode: Mode,
    },
    Equals {
        p1_mode: Mode,
        p2_mode: Mode,
        p3_mode: Mode,
    },
    End,
    Unknown,
}

impl OpCode {
    fn from_i32(v: i32) -> OpCode {
        let code = v % 100;

        let p1_mode = if (v / 100) % 10 == 0 {
            Mode::Position
        } else {
            Mode::Immediate
        };

        let p2_mode = if (v / 1000) % 10 == 0 {
            Mode::Position
        } else {
            Mode::Immediate
        };

        let p3_mode = if (v / 10000) % 10 == 0 {
            Mode::Position
        } else {
            Mode::Immediate
        };

        match code {
            1 => OpCode::Add {
                p1_mode,
                p2_mode,
                p3_mode,
            },
            2 => OpCode::Mut {
                p1_mode,
                p2_mode,
                p3_mode,
            },
            3 => OpCode::Input,
            4 => OpCode::Output { p1_mode },
            5 => OpCode::JumpIfTrue { p1_mode, p2_mode },
            6 => OpCode::JumpIfFalse { p1_mode, p2_mode },
            7 => OpCode::LessThan {
                p1_mode,
                p2_mode,
                p3_mode,
            },
            8 => OpCode::Equals {
                p1_mode,
                p2_mode,
                p3_mode,
            },
            99 => OpCode::End,
            _ => OpCode::Unknown,
        }
    }
}

fn load_input(path: PathBuf) -> Result<Vec<i32>, String> {
    let input_raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let input_raw_split: Vec<&str> = input_raw.split_terminator(',').collect();

    input_raw_split
        .into_iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>())
        .map(|m| m.map_err(|e| e.to_string()))
        .collect::<Result<Vec<i32>, String>>()
}

fn get_value_for_mode(input_state: &[i32], mode: Mode, pos: usize) -> Result<i32, String> {
    let v = match mode {
        Mode::Position => {
            let pos_translated = usize::try_from(
                *input_state
                    .get(pos)
                    .ok_or_else(|| "pos_translated error".to_owned())?,
            )
            .map_err(|e| e.to_string())?;

            *input_state
                .get(pos_translated)
                .ok_or_else(|| "val error".to_owned())?
        }
        Mode::Immediate => *input_state
            .get(pos)
            .ok_or_else(|| "pos_translated error".to_owned())?,
    };

    Ok(v)
}

fn get_next_state(
    input_state: &[i32],
    ip: usize,
    p1_mode: Mode,
    p2_mode: Mode,
    p3_mode: Mode,
    f: &dyn Fn(i32, i32) -> i32,
) -> Result<Vec<i32>, String> {
    if p3_mode == Mode::Immediate {
        return Err("p3_mode is immediate".to_owned());
    }

    let mut next_state = input_state.to_owned();
    let val1: i32 = get_value_for_mode(&input_state, p1_mode, ip + 1)?;
    let val2: i32 = get_value_for_mode(&input_state, p2_mode, ip + 2)?;

    let new_value = f(val1, val2);

    let target_pos = usize::try_from(
        *input_state
            .get(ip + 3)
            .ok_or_else(|| "target_pos error".to_owned())?,
    )
    .map_err(|e| e.to_string())?;

    next_state
        .get_mut(target_pos)
        .map(|v| *v = new_value)
        .ok_or_else(|| "target_pos write error".to_owned())?;

    Ok(next_state)
}

fn get_next_state_input(
    input_state: &[i32],
    input_value: i32,
    pos: usize,
) -> Result<Vec<i32>, String> {
    let mut next_state = input_state.to_owned();

    let target_pos = usize::try_from(
        *input_state
            .get(pos)
            .ok_or_else(|| "target_pos error".to_owned())?,
    )
    .map_err(|e| e.to_string())?;

    next_state
        .get_mut(target_pos)
        .map(|v| *v = input_value)
        .ok_or_else(|| "target_pos write error".to_owned())?;

    Ok(next_state)
}

#[allow(dead_code)]
fn conditional_jump(
    input_state: &[i32],
    ip: usize,
    p1_mode: Mode,
    p2_mode: Mode,
    jump_if_true: bool,
) -> Result<usize, String> {
    let val1: i32 = get_value_for_mode(&input_state, p1_mode, ip + 1)?;
    let val2: i32 = get_value_for_mode(&input_state, p2_mode, ip + 2)?;

    let do_jump = if jump_if_true { val1 != 0 } else { val1 == 0 };

    if do_jump {
        let valid_address =
            usize::try_from(val2).map_err(|_e| format!("invalid jump address: {}", val2))?;
        return Ok(valid_address);
    }

    Ok(ip + 3)
}

fn calc_next_state(
    input_state: &[i32],
    ip: usize,
    input_value: i32,
) -> Result<(Vec<i32>, usize, Option<i32>, bool), String> {
    let op_code_val = input_state
        .get(ip)
        .ok_or_else(|| "op_code_val error".to_owned())?;
    let op_code = OpCode::from_i32(*op_code_val);

    match op_code {
        OpCode::End => return Ok((input_state.to_owned(), ip, None, true)),
        OpCode::Unknown => return Err("Unknown opcode".to_owned()),
        _ => (),
    }

    let next_state: Vec<i32> = match &op_code {
        OpCode::Add {
            p1_mode,
            p2_mode,
            p3_mode,
        } => get_next_state(&input_state, ip, *p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
            v1 + v2
        })?,
        OpCode::Mut {
            p1_mode,
            p2_mode,
            p3_mode,
        } => get_next_state(&input_state, ip, *p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
            v1 * v2
        })?,
        OpCode::LessThan {
            p1_mode,
            p2_mode,
            p3_mode,
        } => get_next_state(&input_state, ip, *p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
            if v1 < v2 {
                1
            } else {
                0
            }
        })?,
        OpCode::Equals {
            p1_mode,
            p2_mode,
            p3_mode,
        } => get_next_state(&input_state, ip, *p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
            if v1 == v2 {
                1
            } else {
                0
            }
        })?,
        OpCode::Input => get_next_state_input(&input_state, input_value, ip + 1)?,
        _ => input_state.to_owned(),
    };

    let output: Option<i32> = match op_code {
        OpCode::Output { p1_mode } => {
            let v = get_value_for_mode(&next_state, p1_mode, ip + 1)?;
            Some(v)
        }
        _ => None,
    };

    let next_ip = match op_code {
        OpCode::Add { .. }
        | OpCode::Mut { .. }
        | OpCode::LessThan { .. }
        | OpCode::Equals { .. } => ip + 4,
        OpCode::Input | OpCode::Output { .. } => ip + 2,
        OpCode::JumpIfFalse { p1_mode, p2_mode } => {
            conditional_jump(&next_state, ip, p1_mode, p2_mode, false)?
        }
        OpCode::JumpIfTrue { p1_mode, p2_mode } => {
            conditional_jump(&next_state, ip, p1_mode, p2_mode, true)?
        }
        _ => ip,
    };

    Ok((next_state, next_ip, output, false))
}

fn run_program(input: Vec<i32>, input_value: i32) -> Result<i32, String> {
    let mut ip: usize = 0;
    let mut state = input;
    let mut last_output: Option<i32> = None;

    loop {
        let (next_state, next_ip, output, done) = calc_next_state(&state, ip, input_value)?;

        if done {
            break;
        }

        if let Some(lo) = last_output {
            if lo != 0 {
                return Err(format!("last output != 0: ip: {} lo: {}", ip, lo).to_owned());
            }
        }

        last_output = output;
        state = next_state;
        ip = next_ip;
    }

    match last_output {
        Some(output) => Ok(output),
        None => Err("no last output".to_owned()),
    }
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let input = load_input(PathBuf::from(INPUT_PATH))?;

    let output = run_program(input, 5)?;

    println!("Output: {}", output);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_op_code_test_add_correct_output() {
        let t: &[i32] = &[1, 0, 0, 0, 99];
        assert_eq!(
            OpCode::from_i32(t[0]),
            OpCode::Add {
                p1_mode: Mode::Position,
                p2_mode: Mode::Position,
                p3_mode: Mode::Position
            }
        );

        let t2: &[i32] = &[101, 0, 0, 0, 99];
        assert_eq!(
            OpCode::from_i32(t2[0]),
            OpCode::Add {
                p1_mode: Mode::Immediate,
                p2_mode: Mode::Position,
                p3_mode: Mode::Position
            }
        );
    }
}
