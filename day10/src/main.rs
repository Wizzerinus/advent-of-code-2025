use lpsolve::Problem;
use std::{fs::File, io::Read, str::FromStr};

#[derive(Default, Debug)]
struct Machine {
    shape: Vec<bool>,
    toggles: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Machine::default();
        let success = s
            .split_whitespace()
            .map(|substr: &str| -> Option<()> {
                if let Some(stripped) = substr.strip_prefix('(') {
                    out.toggles.push(
                        stripped
                            .strip_suffix(')')?
                            .split(',')
                            .flat_map(|x| x.parse())
                            .collect(),
                    );
                } else if let Some(stripped) = substr.strip_prefix('[') {
                    out.shape = stripped
                        .strip_suffix(']')?
                        .as_bytes()
                        .iter()
                        .map(|y| *y == b'#')
                        .collect();
                } else if let Some(stripped) = substr.strip_prefix('{') {
                    out.joltage = stripped
                        .strip_suffix('}')?
                        .split(',')
                        .flat_map(|x| x.parse())
                        .collect();
                } else {
                    return None;
                }

                Some(())
            })
            .all(|q| q.is_some());

        out.toggles.sort_by_key(|x| x.len());
        if !success { Err(s.to_owned()) } else { Ok(out) }
    }
}

fn toggle(shape: &mut [bool], toggle: &[usize]) {
    for i in toggle {
        shape[*i] = !shape[*i];
    }
}

fn count_toggles_rec(shape: &mut [bool], toggles: &[Vec<usize>]) -> Option<usize> {
    if toggles.is_empty() {
        if shape.iter().all(|x| !*x) {
            Some(0)
        } else {
            None
        }
    } else {
        let fst = count_toggles_rec(shape, &toggles[1..]);
        toggle(shape, &toggles[0]);
        let snd = count_toggles_rec(shape, &toggles[1..]);
        toggle(shape, &toggles[0]);
        match (fst, snd) {
            (None, None) => None,
            (None, Some(k)) => Some(k + 1),
            (Some(k), None) => Some(k),
            (Some(m), Some(n)) => Some(m.min(n + 1)),
        }
    }
}

fn count_toggles(m: &Machine) -> usize {
    let mut shape = m.shape.clone();
    count_toggles_rec(&mut shape, &m.toggles).unwrap()
}

fn count_joltage_lp(joltage: &[usize], toggles: &[Vec<usize>]) -> Option<usize> {
    // Seriously, why is an Advent of Code problem seemingly NP-complete?
    //
    let mut problem = Problem::builder()
        .rows(joltage.len() as i32)
        .cols(toggles.len() as i32)
        .objective(&(vec![1.0; toggles.len()]))
        .non_negative_integers()
        .minimize();
    for (i, v) in joltage.iter().enumerate() {
        let mut constraint = vec![0.0f64; toggles.len()];
        for (j, t) in toggles.iter().enumerate() {
            if t.contains(&i) {
                constraint[j] = 1.0;
            }
        }
        problem = problem.constraint(&constraint, lpsolve::ConstraintType::Eq, *v as f64);
    }
    let sol = problem.solve().ok()?;
    assert!(sol.is_optimal());
    sol.variables()?
        .iter()
        .map(|x| *x as usize)
        .reduce(|x, y| x + y)
}

fn count_joltage(m: &Machine) -> usize {
    count_joltage_lp(&m.joltage, &m.toggles).unwrap()
}

fn run(machines: &[Machine], day: usize, func: fn(&Machine) -> usize) {
    println!(
        "Day {}: {}",
        day,
        machines.iter().map(func).reduce(|x, y| x + y).unwrap()
    )
}

fn main() {
    let mut file = File::open("./input10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let machines: Vec<Machine> = contents.split("\n").map(|x| x.parse().unwrap()).collect();

    // Day 1 is after day 2 because lpsolve outputs a lot of junk.
    run(&machines, 2, count_joltage);
    run(&machines, 1, count_toggles);
}
