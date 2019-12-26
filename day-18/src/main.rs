use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

type Pos = (usize, usize);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Map {
    walls: HashSet<Pos>,
    keys: HashMap<Pos, char>,
    doors: HashMap<Pos, char>,
    start: Pos,
}

const START: char = '@';
const WALL: char = '#';
const PATH: char = '.';

impl FromStr for Map {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, String> {
        let mut walls: HashSet<Pos> = HashSet::new();
        let mut keys: HashMap<Pos, char> = HashMap::new();
        let mut doors: HashMap<Pos, char> = HashMap::new();

        let mut start: Option<Pos> = None;

        raw.lines()
            .map(|l| l.trim())
            .enumerate()
            .for_each(|(y, l)| {
                l.chars().enumerate().for_each(|(x, c)| match c {
                    WALL => {
                        walls.insert((x, y));
                    }
                    'a'..='z' => {
                        keys.insert((x, y), c);
                    }
                    'A'..='Z' => {
                        doors.insert((x, y), c.to_ascii_lowercase());
                    }
                    START => {
                        start = Some((x, y));
                    }
                    PATH => {}
                    _ => panic!("unknown char: {} at ({}, {})", c, x, y),
                })
            });

        if start.is_none() {
            return Err("no start position found".to_string());
        }

        Ok(Map {
            walls,
            keys,
            doors,
            start: start.unwrap(),
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct MazePos {
    pub pos: Pos,
    pub required_keys: BTreeSet<char>,
}

const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl MazePos {
    fn get_direction_neighbor(&self, direction: (i8, i8), map: &Map) -> Option<MazePos> {
        // calc new pos
        let pos: Pos = (
            (self.pos.0 as i64 + direction.0 as i64) as usize,
            (self.pos.1 as i64 + direction.1 as i64) as usize,
        );

        // is new pos is a wall there is no neighbor
        if map.walls.get(&pos).is_some() {
            return None;
        }

        // if new pos is a door and we do not have a key return None
        if let Some(d) = map.doors.get(&pos) {
            if self.required_keys.get(d).is_some() {
                return None;
            }
        }

        let mut required_keys = self.required_keys.clone();

        // if new pos is a key, remove from required keys
        if let Some(k) = map.keys.get(&pos) {
            required_keys.remove(&k);
        }

        Some(MazePos { pos, required_keys })
    }

    pub fn get_neighbors(&self, map: &Map) -> Vec<(MazePos, usize)> {
        DIRECTIONS
            .iter()
            .filter_map(|d| self.get_direction_neighbor(*d, &map))
            .map(|p| (p, 1))
            .collect::<Vec<(MazePos, usize)>>()
    }
}

fn find_shortest_path(map: &Map) -> Option<(std::vec::Vec<MazePos>, usize)> {
    let required_keys: BTreeSet<char> = map.keys.values().copied().collect();
    let start = MazePos {
        pos: map.start,
        required_keys,
    };

    dijkstra(
        &start,
        |p: &MazePos| p.get_neighbors(&map),
        |p: &MazePos| p.required_keys.is_empty(),
    )
}

fn load_map(path: &PathBuf) -> Result<Map, String> {
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Map::from_str(&raw)
}

const INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let map = load_map(&PathBuf::from(INPUT_PATH));

    let path = find_shortest_path(&map.unwrap()).ok_or_else(|| "no way found".to_owned())?;
    println!("{:?}", path.1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_parsing_map1_valid() {
        let example_map_1 = r"########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################";

        let map = Map::from_str(example_map_1);
        assert!(map.is_ok());

        let map = map.unwrap();
        assert_eq!(map.walls.len(), 75);
        assert_eq!(map.keys.len(), 6);
        assert_eq!(map.doors.len(), 5);
    }
}
