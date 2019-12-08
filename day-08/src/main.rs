use std::collections::{HashMap, HashSet};
use std::fs;

use std::path::PathBuf;

fn load_input(path: PathBuf) -> Result<Vec<char>, String> {
    let input_raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let chars = input_raw.trim().chars().collect();

    Ok(chars)
}

fn into_layers(raw: Vec<char>, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let block_size = rows * cols;
    raw.chunks(block_size).map(|w| w.to_owned()).collect()
}

fn count_pixels(layer: &[char]) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    layer.iter().for_each(|c| *map.entry(*c).or_insert(0) += 1);

    map
}

#[allow(dead_code)]
fn get_unique_pixels(image: &[char]) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();

    image.iter().for_each(|c| {
        set.insert(*c);
    });

    set
}

fn calc_checksum(layers: &[Vec<char>]) -> Result<usize, String> {
    let mut pixel_counts: Vec<HashMap<char, usize>> =
        layers.iter().map(|l| count_pixels(l)).collect();

    let layers_without_0 = pixel_counts
        .iter()
        .filter(|m| m.get(&'0').is_none())
        .count();

    if layers_without_0 > 0 {
        return Err("there are layers without a 0 pixel".to_owned());
    }

    pixel_counts.sort_by(|x, y| x.get(&'0').unwrap().cmp(y.get(&'0').unwrap()));

    let min = pixel_counts
        .get(0)
        .ok_or_else(|| "pixel count min error".to_owned())?;

    let min_1 = min
        .get(&'1')
        .ok_or_else(|| "1 not found for min 0 layer".to_owned())?;
    let min_2 = min
        .get(&'2')
        .ok_or_else(|| "2 not found for min 0 layer".to_owned())?;

    Ok(min_1 * min_2)
}

fn calc_merged_layer(layers: &[Vec<char>], rows: usize, cols: usize) -> Result<Vec<char>, String> {
    let block_size = rows * cols;
    let mut final_layer = vec!['X'; block_size];

    for p in 0..block_size {
        for (i, l) in layers.iter().enumerate() {
            let v = l
                .get(p)
                .ok_or_else(|| format!("Could not get <{}> vor layer <{}>", p, i).to_owned())?;
            match v {
                '0' => {
                    *final_layer.get_mut(p).unwrap() = ' ';
                    break;
                }
                '1' => {
                    *final_layer.get_mut(p).unwrap() = '#';
                    break;
                }
                '2' => {
                    // do nothing
                }
                _ => {
                    return Err(
                        format!("calc merged layer with unknown pixel type <{}>", v).to_owned()
                    )
                }
            }
        }
    }

    Ok(final_layer)
}

fn print_layer(layer: &[char], cols: usize) {
    let layer_with_cols: Vec<Vec<char>> = layer.chunks(cols).map(|c| c.to_owned()).collect();
    let layer_with_col_strings: Vec<String> = layer_with_cols
        .into_iter()
        .map(|c| c.into_iter().collect())
        .collect();

    layer_with_col_strings
        .iter()
        .for_each(|s| println!("{}", s));
}

static INPUT_PATH: &str = "input/input.txt";
static INPUT_ROWS: usize = 6;
static INPUT_COLS: usize = 25;

#[allow(dead_code)]
static INPUT_PATH_TEST: &str = "input/input_test.txt";
#[allow(dead_code)]
static INPUT_TEST_ROWS: usize = 2;
#[allow(dead_code)]
static INPUT_TEST_COLS: usize = 2;

fn main() -> Result<(), String> {
    let input = load_input(PathBuf::from(INPUT_PATH))?;

    let layers = into_layers(input, INPUT_ROWS, INPUT_COLS);

    let checksum = calc_checksum(&layers)?;

    println!("Checksum: {}", checksum);

    let merged_layer = calc_merged_layer(&layers, INPUT_ROWS, INPUT_COLS)?;

    print_layer(&merged_layer, 25);

    Ok(())
}
