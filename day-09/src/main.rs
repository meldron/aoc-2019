#![allow(dead_code)]
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
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
    fn get_mode(v: i64) -> Mode {
        match v {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!(format!("unknown mode: {}", v)),
        }
    }

    fn from_i64(v: i64) -> OpCode {
        let code = v % 100;

        let p1_mode = OpCode::get_mode((v / 100) % 10);
        let p2_mode = OpCode::get_mode((v / 1000) % 10);
        let p3_mode = OpCode::get_mode((v / 10000) % 10);

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IntCode {
    state: Vec<i64>,
    steps: usize,
    ip: usize,
    rb: isize,
    input_value: i64,
    done: bool,
    outputs: Vec<i64>,
    op_codes: Vec<OpCode>,
}

impl IntCode {
    pub fn new(input_state: Vec<i64>, input_value: i64) -> Self {
        IntCode {
            state: input_state,
            steps: 0,
            ip: 0,
            rb: 0,
            input_value,
            done: false,
            outputs: Vec::new(),
            op_codes: Vec::new(),
        }
    }

    pub fn load(path: &PathBuf, input_value: i64) -> Result<Self, String> {
        let input = IntCode::load_input(path)?;

        Ok(IntCode::new(input, input_value))
    }

    fn load_input(path: &PathBuf) -> Result<Vec<i64>, String> {
        let input_raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let input_raw_split: Vec<&str> = input_raw.split_terminator(',').collect();

        input_raw_split
            .into_iter()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<i64>())
            .map(|m| m.map_err(|e| e.to_string()))
            .collect::<Result<Vec<i64>, String>>()
    }

    pub fn run(&mut self) -> Result<i64, String> {
        let mut last_output: Option<i64> = None;

        loop {
            let output = self.step()?;

            if self.done {
                break;
            }

            if let Some(lo) = last_output {
                if lo != 0 {
                    return Err(format!("last output != 0: ip: {} lo: {}", self.ip, lo).to_owned());
                }
            }

            last_output = output;
        }

        self.outputs
            .last()
            .ok_or_else(|| "no last output".to_owned())
            .map(|o| *o)
    }

    fn step(&mut self) -> Result<Option<i64>, String> {
        let op_code_val = self
            .state
            .get(self.ip)
            .ok_or_else(|| "op_code_val error".to_owned())?;
        let op_code = OpCode::from_i64(*op_code_val);

        let mut output: Option<i64> = None;

        match op_code {
            OpCode::End => {
                self.done = true;
                return Ok(None);
            }
            OpCode::Output { p1_mode } => {
                let o = self.get_value_for_mode(p1_mode, self.ip + 1)?;
                self.outputs.push(o);
                output = Some(o);
            }
            OpCode::Unknown => return Err("Unknown opcode".to_owned()),
            _ => (),
        }

        let next_state: Vec<i64> = match &op_code {
            OpCode::Add {
                p1_mode,
                p2_mode,
                p3_mode,
            } => self.calc_next_state(*p1_mode, *p2_mode, *p3_mode, &|v1, v2| v1 + v2)?,
            OpCode::Mut {
                p1_mode,
                p2_mode,
                p3_mode,
            } => self.calc_next_state(*p1_mode, *p2_mode, *p3_mode, &|v1, v2| v1 * v2)?,
            OpCode::LessThan {
                p1_mode,
                p2_mode,
                p3_mode,
            } => self.calc_next_state(*p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
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
            } => self.calc_next_state(*p1_mode, *p2_mode, *p3_mode, &|v1, v2| {
                if v1 == v2 {
                    1
                } else {
                    0
                }
            })?,
            OpCode::Input => self.use_input_for_next_state(self.ip + 1)?,
            _ => self.state.to_owned(),
        };

        let next_ip = self.calc_next_ip(&op_code)?;

        self.state = next_state;
        self.ip = next_ip;
        self.steps += 1;
        self.op_codes.push(op_code);

        Ok(output)
    }

    fn calc_next_ip(&self, op_code: &OpCode) -> Result<usize, String> {
        let next_ip = match op_code {
            OpCode::Add { .. }
            | OpCode::Mut { .. }
            | OpCode::LessThan { .. }
            | OpCode::Equals { .. } => self.ip + 4,
            OpCode::Input | OpCode::Output { .. } => self.ip + 2,
            OpCode::JumpIfFalse { p1_mode, p2_mode } => {
                self.conditional_jump(*p1_mode, *p2_mode, false)?
            }
            OpCode::JumpIfTrue { p1_mode, p2_mode } => {
                self.conditional_jump(*p1_mode, *p2_mode, true)?
            }
            _ => self.ip,
        };

        Ok(next_ip)
    }

    fn conditional_jump(
        &self,
        p1_mode: Mode,
        p2_mode: Mode,
        jump_if_true: bool,
    ) -> Result<usize, String> {
        let val1: i64 = self.get_value_for_mode(p1_mode, self.ip + 1)?;
        let val2: i64 = self.get_value_for_mode(p2_mode, self.ip + 2)?;

        let do_jump = if jump_if_true { val1 != 0 } else { val1 == 0 };

        if do_jump {
            let valid_address =
                usize::try_from(val2).map_err(|_e| format!("invalid jump address: {}", val2))?;
            return Ok(valid_address);
        }

        Ok(self.ip + 3)
    }

    fn use_input_for_next_state(&self, pos: usize) -> Result<Vec<i64>, String> {
        let mut next_state = self.state.to_owned();

        let target_pos = usize::try_from(
            *self
                .state
                .get(pos)
                .ok_or_else(|| "target_pos error".to_owned())?,
        )
        .map_err(|e| e.to_string())?;

        next_state
            .get_mut(target_pos)
            .map(|v| *v = self.input_value)
            .ok_or_else(|| "target_pos write error".to_owned())?;

        Ok(next_state)
    }

    fn calc_next_state(
        &self,
        p1_mode: Mode,
        p2_mode: Mode,
        p3_mode: Mode,
        f: &dyn Fn(i64, i64) -> i64,
    ) -> Result<Vec<i64>, String> {
        if p3_mode == Mode::Immediate {
            return Err("p3_mode is immediate".to_owned());
        }

        let mut next_state = self.state.to_owned();
        let val1: i64 = self.get_value_for_mode(p1_mode, self.ip + 1)?;
        let val2: i64 = self.get_value_for_mode(p2_mode, self.ip + 2)?;

        let new_value = f(val1, val2);

        let target_pos = usize::try_from(
            *self
                .state
                .get(self.ip + 3)
                .ok_or_else(|| "target_pos error".to_owned())?,
        )
        .map_err(|e| e.to_string())?;

        next_state
            .get_mut(target_pos)
            .map(|v| *v = new_value)
            .ok_or_else(|| "target_pos write error".to_owned())?;

        Ok(next_state)
    }

    fn set_value_for_mode(
        state: &[i64],
        mode: Mode,
        pos: usize,
        rb: isize,
        new_value: i64,
    ) -> Result<Vec<i64>, String> {
        let mut next_state = state.to_owned();

        let target_pos: usize = match mode {
            Mode::Position => usize::try_from(
                *state
                    .get(pos)
                    .ok_or_else(|| "target_pos error".to_owned())?,
            )
            .map_err(|e| e.to_string())?,
            Mode::Relative => {
                let offset = *state
                    .get(pos)
                    .ok_or_else(|| "Relative pos_translated error".to_owned())?;

                let relative_pos = rb + offset as isize;

                usize::try_from(relative_pos).map_err(|e| e.to_string())?
            }

            _ => unreachable!("illegal mode for set_value_for_mode"),
        };

        if target_pos >= next_state.len() {
            next_state.resize(target_pos + 1, 0);
        }

        next_state
            .get_mut(target_pos)
            .map(|v| *v = new_value)
            .ok_or_else(|| "target_pos write error".to_owned())?;

        Ok(next_state)
    }

    // get value for a specific pos according to the mode
    fn get_value_for_mode(&self, mode: Mode, pos: usize) -> Result<i64, String> {
        let pos_translated: usize = match mode {
            Mode::Position => usize::try_from(
                *self
                    .state
                    .get(pos)
                    .ok_or_else(|| "pos_translated error".to_owned())?,
            )
            .map_err(|e| e.to_string())?,
            Mode::Immediate => pos,
            Mode::Relative => {
                let offset = *self
                    .state
                    .get(pos)
                    .ok_or_else(|| "Relative pos_translated error".to_owned())?;

                let relative_pos = self.rb + offset as isize;

                usize::try_from(relative_pos).map_err(|e| e.to_string())?
            }
        };

        let v = *self.state.get(pos_translated).unwrap_or(&0);

        Ok(v)
    }
}

static INPUT_PATH: &str = "input/input_5.txt";

fn main() -> Result<(), String> {
    let mut int_code = IntCode::load(&PathBuf::from(INPUT_PATH), 5)?;
    let output = int_code.run()?;

    println!("Output: {}", output);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_op_code_test_add_correct_output() {
        let t: &[i64] = &[1, 0, 0, 0, 99];
        assert_eq!(
            OpCode::from_i64(t[0]),
            OpCode::Add {
                p1_mode: Mode::Position,
                p2_mode: Mode::Position,
                p3_mode: Mode::Position
            }
        );

        let t2: &[i64] = &[101, 0, 0, 0, 99];
        assert_eq!(
            OpCode::from_i64(t2[0]),
            OpCode::Add {
                p1_mode: Mode::Immediate,
                p2_mode: Mode::Position,
                p3_mode: Mode::Position
            }
        );
    }
}
