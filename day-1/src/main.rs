use aoc::{
    anyhow::{self, anyhow, Context},
    wrap_main, Challenge,
};

#[derive(Default)]
struct Reader {
    current_calories: usize,
    elves: Vec<usize>,
}

impl Reader {
    fn add_calories(&mut self, how_many: usize) {
        self.current_calories += how_many;
    }

    fn flush(&mut self) {
        if self.current_calories > 0 {
            self.elves.push(self.current_calories);
            self.current_calories = 0;
        }
    }
}

fn anyhow_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut reader = Reader::default();
    for line in challenge.input.lines() {
        if line.is_empty() {
            reader.flush();
        } else {
            let calories = line.parse::<usize>().context("parse number of calories")?;
            reader.add_calories(calories);
        }
    }
    reader.flush();
    let elves = reader.elves;

    let part_1 = elves
        .iter()
        .max()
        .ok_or_else(|| anyhow!("no lines in input file?"))?;
    println!("part 1: {part_1}");

    let mut elves = elves;
    elves.sort_by(|a, b| a.cmp(&b).reverse());
    let part_2: usize = elves.iter().take(3).sum();
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(anyhow_main)
}
