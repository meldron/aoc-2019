use std::collections::HashMap;
use std::path::PathBuf;

mod int_code;

use int_code::IntCode;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl Color {
    pub fn from_i64(i: i64) -> Result<Self, String> {
        match i {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(format!("Unknown color: {}", i)),
        }
    }

    pub fn as_i64(self) -> i64 {
        match &self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    pub fn as_pixel(self) -> char {
        match &self {
            Color::Black => ' ',
            Color::White => '#',
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn from_i64(i: i64) -> Result<Self, String> {
        match i {
            0 => Ok(TurnDirection::Left),
            1 => Ok(TurnDirection::Right),
            _ => Err(format!("Unknown color: {}", i)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match &self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    pub fn turn_right(self) -> Self {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

type Point = (i64, i64);

type HullMap = HashMap<Point, (Color, usize)>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HullRobot {
    map: HullMap,
    x: i64,
    y: i64,
    direction: Direction,
}

impl Default for HullRobot {
    fn default() -> Self {
        Self::new()
    }
}

impl HullRobot {
    pub fn new() -> Self {
        HullRobot {
            map: HashMap::new(),
            x: 0,
            y: 0,
            direction: Direction::Up,
        }
    }

    pub fn new_with_data(data: &[(Point, Color)]) -> Self {
        let mut robot = HullRobot::new();
        data.iter().for_each(|(p, c)| {
            robot.map.insert(*p, (*c, 0));
        });

        robot
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_to_next_pos(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn get_current_color(&mut self) -> Color {
        self.map.entry((self.x, self.y)).or_default().0
    }

    fn paint_and_move(&mut self, new_color: Color, turn_direction: TurnDirection) {
        let (color, visits) = self.map.entry((self.x, self.y)).or_default();

        *color = new_color;
        *visits += 1;

        match turn_direction {
            TurnDirection::Left => self.turn_left(),
            TurnDirection::Right => self.turn_right(),
        };

        self.move_to_next_pos();
    }

    pub fn get_painted_once(&self) -> usize {
        self.map.len()
    }

    pub fn paint_ship(&mut self, int_code: &mut IntCode) -> Result<usize, String> {
        loop {
            let input = self.get_current_color().as_i64();

            let output = int_code.run_for_two_outputs(input)?;

            if output.is_none() {
                break;
            }

            let new_color = Color::from_i64(output.unwrap().0)?;
            let turn_direction = TurnDirection::from_i64(output.unwrap().1)?;

            self.paint_and_move(new_color, turn_direction);
        }

        let painted_once = self.get_painted_once();

        Ok(painted_once)
    }

    pub fn get_painted_coords_system(&self) -> Result<(Point, Point), String> {
        let y_min = self
            .map
            .keys()
            .map(|(_, y)| y)
            .min()
            .ok_or_else(|| "no y_min".to_owned())?;
        let y_max = self
            .map
            .keys()
            .map(|(_, y)| y)
            .max()
            .ok_or_else(|| "no y_max".to_owned())?;

        let x_min = self
            .map
            .keys()
            .map(|(x, _)| x)
            .min()
            .ok_or_else(|| "no x_min".to_owned())?;
        let x_max = self
            .map
            .keys()
            .map(|(x, _)| x)
            .max()
            .ok_or_else(|| "no x_max".to_owned())?;

        Ok(((*x_min, *x_max), (*y_min, *y_max)))
    }

    pub fn print_painted_hull(&mut self) -> Result<(), String> {
        let ((x_min, x_max), (y_min, y_max)) = self.get_painted_coords_system()?;

        let output: Vec<String> = (y_min..=y_max)
            .map(|y| {
                (x_min..=x_max)
                    .map(|x| {
                        self.map.entry((x, y)).or_default().0.as_pixel()
                    })
                    .collect()
            })
            .collect();

        println!("{}", output.join("\n"));

        Ok(())
    }
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let mut hull_robot = HullRobot::new();
    let mut int_code = IntCode::load(&PathBuf::from(INPUT_PATH), None)?;

    let painted_once = hull_robot.paint_ship(&mut int_code)?;

    println!("painted once: {}", painted_once);

    let mut hull_robot2 = HullRobot::new_with_data(&[((0, 0), Color::White)]);
    let mut int_code2 = IntCode::load(&PathBuf::from(INPUT_PATH), None)?;

    let painted_once2 = hull_robot2.paint_ship(&mut int_code2)?;

    println!("painted once: {}", painted_once2);
    println!("{:?}", hull_robot2.get_painted_coords_system()?);
    hull_robot2.print_painted_hull()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use int_code::{Mode, OpCode};

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

    #[test]
    fn test_program_quine() {
        let state: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut int_code = IntCode::new(state.clone(), Some(0));

        int_code.set_ignore_outputs(true);

        let output = int_code.run_complete_program();
        assert!(output.is_ok());

        let all_outputs = int_code.get_all_outputs();
        assert_eq!(*all_outputs, state);
    }

    #[test]
    fn test_program_middle() {
        let mut int_code = IntCode::new(vec![104, 1_125_899_906_842_624, 99], Some(0));
        let output = int_code.run_complete_program();
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), 1_125_899_906_842_624);
    }

    #[test]
    fn test_program_16_digit() {
        let mut int_code =
            IntCode::new(vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], Some(0));
        let output = int_code.run_complete_program();
        assert!(output.is_ok());
        assert_eq!(output.unwrap().to_string().len(), 16);
    }
}
