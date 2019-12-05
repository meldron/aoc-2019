use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Command {
    direction: Direction,
    steps: usize,
}

impl Command {
    fn from_str(s: &str) -> Result<Command, String> {
        let direction_raw = s
            .chars()
            .nth(0)
            .ok_or_else(|| format!("direction char not found: {}", s).to_owned())?;

        let direction = match direction_raw {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => return Err(format!("Unknown direction <{}> for <{}>", direction_raw, s)),
        };

        let steps = (&s[1..]).parse::<usize>().map_err(|e| {
            format!("error parsing steps <{}> for <{}>: {}", &s[1..], s, e).to_owned()
        })?;

        Ok(Command { direction, steps })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Wire {
    head: (i32, i32),
    moves: usize,
    pos_moves: HashMap<(i32, i32), usize>,
    visited: HashSet<(i32, i32)>,
}

impl Wire {
    fn new() -> Self {
        Wire {
            head: (0, 0),
            moves: 0,
            pos_moves: HashMap::new(),
            visited: HashSet::new(),
        }
    }
    fn from_commands(cs: Vec<Command>) -> Self {
        let mut wire = Wire::new();

        cs.into_iter().for_each(|c| wire.move_head(c));

        wire
    }
    fn move_head(&mut self, c: Command) {
        let d = match c.direction {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        for _ in 0..c.steps {
            self.moves += 1;
            let n = (self.head.0 + d.0, self.head.1 + d.1);
            if self.visited.get(&n).is_none() {
                let v = n;
                self.visited.insert(v);
                self.pos_moves.insert(v, self.moves);
            }

            self.head = n;
        }
    }
    fn get_visited(&self) -> Vec<&(i32, i32)> {
        self.visited.iter().collect::<Vec<_>>()
    }
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Grid {
    map: HashMap<(i32, i32), usize>,
    wires: Vec<Wire>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            map: HashMap::<(i32, i32), usize>::new(),
            wires: Vec::new(),
        }
    }

    fn add_wire(&mut self, wire: Wire) {
        wire.get_visited()
            .iter()
            .for_each(|(x, y)| *self.map.entry((*x, *y)).or_insert(0) += 1);
        self.wires.push(wire);
    }

    fn get_intersections(&self) -> Vec<&(i32, i32)> {
        let min_visited = self.wires.len() - 1;

        self.map
            .iter()
            .filter(|(_, v)| **v > min_visited)
            .map(|(p, _)| p)
            .collect()
    }

    fn get_nearest_intersection(&self) -> Option<i32> {
        let intersections = self.get_intersections();
        if intersections.is_empty() {
            return None;
        }

        let distances: Vec<i32> = intersections
            .iter()
            .map(|p| manhattan_distance((0, 0), **p))
            .collect();

        let min_distance = distances.iter().fold(std::i32::MAX, |a, &b| a.min(b));

        Some(min_distance)
    }

    fn get_nearest_intersection_moves(&self) -> Option<usize> {
        let intersections = self.get_intersections();
        if intersections.is_empty() {
            return None;
        }

        let distances: Option<Vec<usize>> = intersections
            .iter()
            .map(|p| self.wires.iter().map(|w| w.pos_moves.get(p)).sum())
            .collect();

        distances.as_ref()?;

        let min_distance = distances
            .unwrap()
            .iter()
            .fold(std::usize::MAX, |a, &b| a.min(b));

        Some(min_distance)
    }
}

fn load_commands(path: PathBuf) -> Result<Vec<Vec<Command>>, String> {
    let f = File::open(path).map_err(|e| e.to_string())?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.map_err(|e| e.to_string()))
        .collect::<Result<Vec<String>, String>>()?;

    let commands_raw = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_terminator(',').map(|m| m.to_owned()).collect())
        .collect::<Vec<Vec<String>>>();

    let commands = commands_raw
        .into_iter()
        .enumerate()
        .map(|(wn, cs)| {
            cs.iter()
                .enumerate()
                .map(|(cn, c)| (cn, Command::from_str(c)))
                .map(|(cn, m)| m.map_err(|e| format!("W#{} C#{}: {}", wn, cn, e)))
                .collect::<Result<Vec<_>, String>>()
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(commands)
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let commands = load_commands(PathBuf::from(INPUT_PATH))?;

    let wires: Vec<Wire> = commands.into_iter().map(Wire::from_commands).collect();

    let mut grid = Grid::new();
    wires.into_iter().for_each(|w| grid.add_wire(w));

    println!("{:?}", grid.get_nearest_intersection());
    println!("{:?}", grid.get_nearest_intersection_moves());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_command_valid_input_valid_command() {
        let input = "R123";
        let command_parsed_result = Command::from_str(input);

        assert!(command_parsed_result.is_ok());

        let command_parsed = command_parsed_result.unwrap();

        let command_expected = Command {
            direction: Direction::Right,
            steps: 123,
        };
        assert_eq!(command_parsed, command_expected);
    }

    #[test]
    fn parse_command_invalid_input_error() {
        let input = "T123";
        let command_parsed_result = Command::from_str(input);

        assert!(command_parsed_result.is_err());

        let command_parsed_error = command_parsed_result.err().unwrap();

        assert_eq!(command_parsed_error, "Unknown direction 'T' for 'T123'");
    }
}
