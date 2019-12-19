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
    Input {
        p1_mode: Mode,
    },
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
    AdjustRelativeBase {
        p1_mode: Mode,
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

    pub fn from_i64(v: i64) -> OpCode {
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
            3 => OpCode::Input { p1_mode },
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
            9 => OpCode::AdjustRelativeBase { p1_mode },
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
    single_input_value: Option<i64>,
    inputs: Vec<i64>,
    done: bool,
    outputs: Vec<i64>,
    op_codes: Vec<OpCode>,
    rb_history: Vec<isize>,
    ignore_outputs: bool,
}

impl IntCode {
    pub fn new(input_state: Vec<i64>, single_input_value: Option<i64>) -> Self {
        IntCode {
            state: input_state,
            steps: 0,
            ip: 0,
            rb: 0,
            single_input_value,
            inputs: Vec::new(),
            done: false,
            outputs: Vec::new(),
            op_codes: Vec::new(),
            rb_history: Vec::new(),
            ignore_outputs: false,
        }
    }

    pub fn load(path: &PathBuf, single_input_value: Option<i64>) -> Result<Self, String> {
        let input = IntCode::load_input(path)?;

        Ok(IntCode::new(input, single_input_value))
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

    pub fn run_for_two_outputs(&mut self, input: i64) -> Result<Option<(i64, i64)>, String> {
        if self.done {
            return Err("run_for_two_outputs called after int code is already done".to_owned());
        }

        self.inputs.push(input);

        let mut outputs: Vec<i64> = Vec::new();

        loop {
            if outputs.len() == 2 {
                return Ok(Some((outputs[0], outputs[1])));
            }

            let output = self.step()?;

            if self.done {
                break;
            }

            if let Some(o) = output {
                outputs.push(o);
            }
        }

        Ok(None)
    }

    pub fn run_complete_program(&mut self) -> Result<i64, String> {
        if self.done {
            return Err("can only be run once".to_owned());
        }

        let mut last_output: Option<i64> = None;

        loop {
            let output = self.step()?;

            if self.done {
                break;
            }

            if let Some(lo) = last_output {
                if !self.ignore_outputs && lo != 0 {
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

    fn adjust_relative_base(&self, mode: Mode, offset: usize) -> Result<isize, String> {
        let adjust_with = self.get_value_for_mode(mode, offset)? as isize;

        Ok(self.rb + adjust_with)
    }

    fn step(&mut self) -> Result<Option<i64>, String> {
        let op_code_val = self
            .state
            .get(self.ip)
            .ok_or_else(|| "op_code_val error".to_owned())?;
        let op_code = OpCode::from_i64(*op_code_val);

        let mut output: Option<i64> = None;
        let mut new_rb: Option<isize> = None;

        match op_code {
            OpCode::End => {
                self.done = true;
                return Ok(None);
            }
            OpCode::Output { p1_mode } => {
                let o = self.get_value_for_mode(p1_mode, 1)?;
                self.outputs.push(o);
                output = Some(o);
            }
            OpCode::AdjustRelativeBase { p1_mode } => {
                new_rb = Some(self.adjust_relative_base(p1_mode, 1)?);
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
            OpCode::Input { p1_mode } => self.use_input_for_next_state(*p1_mode, 1)?,
            _ => self.state.to_owned(),
        };

        let next_ip = self.calc_next_ip(&op_code)?;

        if let Some(nrb) = new_rb {
            self.rb_history.push(self.rb);
            self.rb = nrb;
        }

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
            OpCode::Input { .. } | OpCode::Output { .. } | OpCode::AdjustRelativeBase { .. } => {
                self.ip + 2
            }
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
        let val1: i64 = self.get_value_for_mode(p1_mode, 1)?;
        let val2: i64 = self.get_value_for_mode(p2_mode, 2)?;

        let do_jump = if jump_if_true { val1 != 0 } else { val1 == 0 };

        if do_jump {
            let valid_address =
                usize::try_from(val2).map_err(|_e| format!("invalid jump address: {}", val2))?;
            return Ok(valid_address);
        }

        Ok(self.ip + 3)
    }

    fn use_input_for_next_state(&mut self, mode: Mode, offset: usize) -> Result<Vec<i64>, String> {
        let input = if let Some(single_input_value) = self.single_input_value {
            single_input_value
        } else {
            self.inputs.pop().ok_or_else(|| "no input value available".to_owned())?
        };

        let next_state = self.set_value_for_mode(mode, offset, input)?;

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

        let val1: i64 = self.get_value_for_mode(p1_mode, 1)?;
        let val2: i64 = self.get_value_for_mode(p2_mode, 2)?;

        let new_value = f(val1, val2);

        let next_state = self.set_value_for_mode(p3_mode, 3, new_value)?;

        Ok(next_state)
    }

    fn set_value_for_mode(
        &self,
        mode: Mode,
        offset: usize,
        new_value: i64,
    ) -> Result<Vec<i64>, String> {
        let mut next_state = self.state.to_owned();
        let pos = self.ip + offset;

        let target_pos: usize = self.get_target_pos(mode, pos)?;

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
    fn get_value_for_mode(&self, mode: Mode, offset: usize) -> Result<i64, String> {
        let pos = self.ip + offset;

        let pos_translated: usize = self.get_target_pos(mode, pos)?;

        let v = *self.state.get(pos_translated).unwrap_or(&0);

        Ok(v)
    }

    fn get_target_pos(&self, mode: Mode, pos: usize) -> Result<usize, String> {
        let target_pos: usize = match mode {
            Mode::Position => usize::try_from(
                *self
                    .state
                    .get(pos)
                    .ok_or_else(|| "target_pos error".to_owned())?,
            )
            .map_err(|e| e.to_string())?,
            Mode::Relative => {
                let offset = *self
                    .state
                    .get(pos)
                    .ok_or_else(|| "Relative pos_translated error".to_owned())?;

                let relative_pos = self.rb + offset as isize;

                usize::try_from(relative_pos).map_err(|e| e.to_string())?
            }

            _ => pos,
        };

        Ok(target_pos)
    }

    pub fn set_ignore_outputs(&mut self, v: bool) {
        self.ignore_outputs = v;
    }

    pub fn get_all_outputs(&self) -> &Vec<i64> {
        self.outputs.as_ref()
    }

    pub fn get_steps(&self) -> usize {
        self.steps
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}