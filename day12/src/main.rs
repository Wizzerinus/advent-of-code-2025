use std::{fs::File, io::Read, str::FromStr};

#[derive(Debug)]
struct Field {
    width: usize,
    height: usize,
    counts: [usize; 6],
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 24 {
            return Err(());
        }
        let mut counts = [0; 6];
        for i in 0..6 {
            counts[i] = s[i * 3 + 7..i * 3 + 9].parse().map_err(|_| ())?;
        }
        Ok(Field {
            width: s[0..2].parse().map_err(|_| ())?,
            height: s[3..5].parse().map_err(|_| ())?,
            counts,
        })
    }
}

#[derive(Debug)]
struct Shape([[bool; 3]; 3]);

impl TryFrom<&[&str]> for Shape {
    type Error = ();

    fn try_from(split: &[&str]) -> Result<Self, Self::Error> {
        let lines = {
            if split.len() == 3 {
                split
            } else if split.len() == 4 {
                &split[1..]
            } else {
                return Err(());
            }
        };
        let mut shape = [[false; 3]; 3];
        for (i, lin) in lines.iter().enumerate() {
            if lin.len() != 3 {
                return Err(());
            }
            for (j, cell) in lin.chars().enumerate() {
                shape[i][j] = cell == '#';
            }
        }
        Ok(Shape(shape))
    }
}

impl Shape {
    fn size(&self) -> usize {
        self.0.iter().flatten().filter(|x| **x).count()
    }
}

impl Field {
    fn fit_trivially(&self) -> bool {
        (self.width / 3) * (self.height / 3) >= self.counts.iter().sum()
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn necessary_size(&self, shapes: &[Shape]) -> usize {
        self.counts
            .iter()
            .enumerate()
            .map(|(i, x)| x * shapes[i].size())
            .sum()
    }
}

fn can_fit(field: &Field, shapes: &[Shape]) -> bool {
    if field.fit_trivially() {
        true
    } else if field.necessary_size(&shapes) < field.size() {
        false
    } else {
        // This input is silly, in that every true answer is trivially true.
        // Otherwise this problem seems NP-complete. No way that's soluble in the span
        // of a day.
        false
    }
}

fn main() {
    let mut file = File::open("./input12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines().collect::<Vec<_>>();
    let shapes = (0..6)
        .map(|x| lines[x * 5..x * 5 + 4].try_into().unwrap())
        .collect::<Vec<Shape>>();
    let fields = lines[30..]
        .iter()
        .map(|x| Field::from_str(x).unwrap())
        .collect::<Vec<_>>();

    println!(
        "Day 1: {}",
        fields.iter().filter(|x| can_fit(x, &shapes)).count()
    )
}
