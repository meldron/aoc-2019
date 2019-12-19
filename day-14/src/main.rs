use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;

type ID = u16;
type Amount = usize;
type SubstanceRaw = (String, Amount);
type Substance = (ID, Amount);

static FUEL: ID = 0;
static FUEL_NAME: &str = "FUEL";

static ORE: ID = 1;
static ORE_NAME: &str = "ORE";

static TRILLION: usize = 1_000_000_000_000;

static REACTION_SPLIT: &str = "=>";

pub struct Reaction {
    pub output: Amount,
    pub reactants: Vec<Substance>,
}

type Cookbook = HashMap<ID, Reaction>;
type Queue = VecDeque<Substance>;
type Stockpile = HashMap<ID, Amount>;

fn parse_substance(s: &str) -> Result<SubstanceRaw, String> {
    let raw: Vec<&str> = s.trim().split_whitespace().collect();
    let amount = raw[0]
        .parse::<Amount>()
        .map_err(|e| format!("Could not parse <{}> to amount: {}", raw[0], e))?;

    let substance_name = raw[1].trim().to_owned();

    if amount == 0 {
        return Err(format!("<{}> amount is <0>", substance_name));
    }

    Ok((substance_name, amount))
}

fn parse_line(line: &str) -> Result<((SubstanceRaw, Vec<SubstanceRaw>)), String> {
    let split: Vec<&str> = line.splitn(2, REACTION_SPLIT).collect();
    let product_raw: SubstanceRaw = parse_substance(split[1])?;
    let reactants_raw: Vec<SubstanceRaw> = split[0]
        .split(',')
        .map(|s| parse_substance(s))
        .collect::<Result<Vec<SubstanceRaw>, String>>()?;

    Ok((product_raw, reactants_raw))
}

fn get_id(map: &mut HashMap<String, ID>, next_id: &mut ID, name: String) -> ID {
    *map.entry(name).or_insert_with(|| {
        let id = *next_id;
        *next_id += 1;
        id
    })
}

fn load_cookbook(path: PathBuf) -> Result<Cookbook, String> {
    let mut cookbook: Cookbook = Cookbook::new();

    let mut id_map: HashMap<String, ID> = HashMap::new();
    id_map.insert(FUEL_NAME.to_owned(), FUEL);
    id_map.insert(ORE_NAME.to_owned(), ORE);
    let mut next_id: ID = 2;

    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;

    raw.lines()
        .map(parse_line)
        .collect::<Result<Vec<((SubstanceRaw, Vec<SubstanceRaw>))>, String>>()?
        .into_iter()
        .for_each(|(product_raw, reactants_raw)| {
            let product_name = product_raw.0;
            let product_amount = product_raw.1;

            if product_amount == 0 {
                panic!("0");
            }

            let product_id = get_id(&mut id_map, &mut next_id, product_name);

            let reactants: Vec<Substance> = reactants_raw
                .into_iter()
                .map(|reactant_raw| {
                    let reactant_name = reactant_raw.0;
                    let reactant_id = get_id(&mut id_map, &mut next_id, reactant_name);
                    if reactant_raw.1 == 0 {
                        panic!("0");
                    }

                    (reactant_id, reactant_raw.1)
                })
                .collect();

            cookbook.insert(
                product_id,
                Reaction {
                    output: product_amount,
                    reactants,
                },
            );
        });

    if cookbook.get(&FUEL).is_none() {
        return Err("FUEL not in cookbook".to_owned());
    }

    if cookbook.get(&ORE).is_some() {
        return Err("ORE in cookbook, but it's always a reactant".to_owned());
    }

    Ok(cookbook)
}

fn take_from_stockpile(stockpile: &mut Stockpile, product_id: ID, amount: Amount) -> Amount {
    let stocked = stockpile.entry(product_id).or_insert(0);
    let take = amount.min(*stocked);

    *stocked -= take;

    take
}

fn get_amount_of_ore_to_produce(
    cookbook: &Cookbook,
    product_id: ID,
    amount: Amount,
) -> Result<Amount, String> {
    let mut queue = Queue::new();
    let mut stockpile = Stockpile::new();
    let mut ore_required: Amount = 0;

    queue.push_back((product_id, amount));

    while let Some((product_id, mut amount_required)) = queue.pop_front() {
        amount_required -= take_from_stockpile(&mut stockpile, product_id, amount_required);

        // nothing to produce we could take everything from the stockpile
        if amount_required < 1 {
            continue;
        }

        let reaction = cookbook
            .get(&product_id)
            .ok_or_else(|| format!("could not find <{}> in cookbook", product_id))?;

        // round up how many of this product we have to produce
        let batch_size = (amount_required + reaction.output - 1) / reaction.output;

        reaction
            .reactants
            .iter()
            .for_each(|(reactant_id, reactant_amount_required)| {
                let required = batch_size * reactant_amount_required;

                if *reactant_id == ORE {
                    ore_required += required;
                } else {
                    queue.push_back((*reactant_id, required));
                }
            });

        let additional = batch_size * reaction.output - amount_required;
        *stockpile.entry(product_id).or_insert(0) += additional;
    }

    Ok(ore_required)
}

fn max_product_with_ore(
    cookbook: &Cookbook,
    product_id: ID,
    ore_available: Amount,
) -> Result<Amount, String> {
    let mut start = 1;
    let mut stop = ore_available;
    let mut guess = start;

    while start < stop {
        let step = (stop + start) / 2;
        let ore_needed = get_amount_of_ore_to_produce(&cookbook, FUEL, step)?;

        if ore_needed <= TRILLION {
            guess = step;
            start = step + 1;
        } else {
            stop = step;
        }
    }

    Ok(guess)
}

fn main() -> Result<(), String> {
    let cookbook = load_cookbook(PathBuf::from("input/input.txt"))?;

    let cost_ore_per_fuel = get_amount_of_ore_to_produce(&cookbook, FUEL, 1)?;

    println!("'{}' ORE to produce '1' FUEL", cost_ore_per_fuel);

    let guess = max_product_with_ore(&cookbook, FUEL, TRILLION)?;
    println!("With '{:#?}' ore, we can produce '{}' fuel", TRILLION, guess);

    Ok(())
}
