use std::fs::read_to_string;
use std::path::PathBuf;

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

// fn create_station_map(asteroids: &[(usize, usize)], station: (usize, usize)) {}

static INPUT_PATH: &str = "input/input.txt";

fn main() -> Result<(), String> {
    let asteroids = load_asteroids(&PathBuf::from(INPUT_PATH))?;
    let (station, most_visible) = find_most_visible(&asteroids);

    println!("Most visible <{}> at <{:?}>", most_visible, station);

    Ok(())
}
