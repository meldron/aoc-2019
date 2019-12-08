use std::collections::{HashMap, HashSet, VecDeque};

use std::fs::read_to_string;
use std::path::PathBuf;

fn load_input(path: PathBuf) -> Result<Vec<(String, String)>, String> {
    let input_raw = read_to_string(path).map_err(|e| e.to_string())?;

    let pairs: Vec<(String, String)> = input_raw
        .trim()
        .lines()
        .map(|s| {
            s.splitn(2, ')')
                .map(|x| x)
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .map(|s| (s[0].to_owned(), s[1].to_owned()))
        .collect();

    Ok(pairs)
}

fn create_orbit_map(input: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    input.into_iter().for_each(|(name, orbited_by)| {
        map.entry(name)
            .or_insert_with(Vec::<String>::new)
            .push(orbited_by.clone())
    });

    map
}

fn create_orbit_map_all_edges(input: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    input.into_iter().for_each(|(name, orbited_by)| {
        map.entry(name.clone())
            .or_insert_with(Vec::<String>::new)
            .push(orbited_by.clone());

        map.entry(orbited_by)
            .or_insert_with(Vec::<String>::new)
            .push(name)
    });

    map
}

#[allow(dead_code)]
fn calc_distance(
    fully_connected_input: HashMap<String, Vec<String>>,
    start: &str,
    target: &str,
) -> Result<usize, String> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut distances = HashMap::<&str, usize>::new();
    let mut queue = VecDeque::<&str>::new();

    fully_connected_input
        .get(target)
        .ok_or_else(|| format!("target {} not in input", target))?;

    queue.push_back(start);
    distances.insert(start, 0);

    while !queue.is_empty() {
        let p = queue
            .pop_front()
            .ok_or_else(|| "queue is empty which should not be possible".to_owned())?;
        let p_distance = *distances
            .get(p)
            .ok_or_else(|| format!("<{}> not in distances", p).to_owned())?;

        if p == target {
            // -2 because '(Between the objects they are orbiting - not between YOU and SAN.)'
            return Ok(p_distance - 2);
        }

        if visited.get(p).is_some() {
            continue;
        }

        visited.insert(p);
        let neighbors = fully_connected_input
            .get(p)
            .ok_or_else(|| format!("<{}> not in input", p).to_owned())?;

        neighbors.iter().for_each(|n| {
            distances.insert(n, p_distance + 1);
            queue.push_back(n);
        });
    }

    Err(format!("no connection from <{}> to <{}>", start, target))
}

fn calc_checksum(orbit_map: &HashMap<String, Vec<String>>, poi: &str, depth: usize) -> usize {
    let point_orbited_by = orbit_map.get(poi);

    if point_orbited_by.is_none() {
        return depth;
    }

    let orbiter_distances: Vec<usize> = point_orbited_by
        .unwrap()
        .iter()
        .map(|o| calc_checksum(orbit_map, o, depth + 1))
        .collect();

    let sum: usize = orbiter_distances.iter().sum();

    sum + depth
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let input = load_input(PathBuf::from(&INPUT_PATH))?;
    let input_2 = input.clone();

    let orbit_map = create_orbit_map(input);
    // println!("{}", orbit_map.keys().len());

    let checksum = calc_checksum(&orbit_map, "COM", 0);

    println!("Checksum: {}", checksum);

    let fully_connected_input = create_orbit_map_all_edges(input_2);
    let distance = calc_distance(fully_connected_input, "YOU", "SAN")?;

    println!("Distance: {}", distance);

    Ok(())
}
