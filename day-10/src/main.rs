use std::fs::read_to_string;
use std::path::PathBuf;

use std::collections::HashMap;

fn load_asteroids(path: &PathBuf) -> Result<Vec<(usize, usize)>, String> {
    let input = read_to_string(path).map_err(|e| e.to_string())?;

    let raw: Vec<Vec<char>> = input.split('\n').map(|l| l.chars().collect()).collect();

    let mut asteroids: Vec<(usize, usize)> = Vec::new();

    raw.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, c)| {
            if *c == '#' {
                asteroids.push((x, y));
            }
        })
    });

    Ok(asteroids)
}

fn find_most_visible(asteroids: &[(usize, usize)]) -> ((usize, usize), usize) {
    let mut best: Option<(usize, usize)> = None;
    let mut max_visible = 0;

    asteroids.iter().for_each(|(p_x, p_y)| {
        let mut angles: Vec<f64> = Vec::new();
        asteroids.iter().for_each(|(a_x, a_y)| {
            let dx = *p_x as f64 - *a_x as f64;
            let dy = *p_y as f64 - *a_y as f64;

            let a = dx.atan2(dy);

            if angles.contains(&a) {
                return;
            }

            angles.push(a);
        });

        if angles.len() > max_visible {
            best = Some((*p_x, *p_y));
            max_visible = angles.len();
        }
    });

    (best.expect("no best found"), max_visible)
}

type Angle = isize;
type Location = (usize, usize);
type Distance = i32;

fn manhattan_distance(p1: &Location, p2: &Location) -> i32 {
    (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()
}

fn create_sorted_station_map(
    asteroids: &[Location],
    station: &Location,
) -> HashMap<Angle, Vec<(Location, Distance)>> {
    let mut map: HashMap<Angle, Vec<(Location, Distance)>> = HashMap::new();

    asteroids
        .iter()
        .filter(|l| !(l.0 == station.0 && l.1 == station.1))
        .for_each(|l| {
            let dx = station.0 as f64 - l.0 as f64;
            let dy = station.1 as f64 - l.1 as f64;

            let angle: Angle = -(dx.atan2(dy).to_degrees() * 1000.0) as isize;
            let distance = manhattan_distance(station, l);

            map.entry(angle).or_default().push((*l, distance));
        });

    // sort map vector entries by distance desc so vector.pop returns closest entry
    map.iter_mut()
        .for_each(|(_a, v)| v.sort_by_key(|(_l, d)| -d));

    map
}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let asteroids = load_asteroids(&PathBuf::from(INPUT_PATH))?;
    let (station, most_visible) = find_most_visible(&asteroids);

    println!("Most visible <{}> at <{:?}>", most_visible, station);

    let mut map = create_sorted_station_map(&asteroids, &station);

    let mut map_keys: Vec<isize> = map.keys().copied().collect();
    map_keys.sort();

    let it = map_keys
        .iter()
        .cycle()
        .skip_while(|angle| **angle < 0)
        .filter_map(|angle| map.get_mut(angle).and_then(|v| v.pop()).map(|(l, _)| l));

    for (c, l) in it.enumerate() {
        if c == 199 {
            println!("200th is <{:?}>: {}", l, l.0 * 100 + l.1);
            return Ok(());
        }

        if c > asteroids.len() {
            return Err("too many iterations".to_owned());
        }
    }

    Err("199 not found".to_owned())
}
