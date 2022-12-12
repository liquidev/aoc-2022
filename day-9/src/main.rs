use std::collections::HashSet;

use aoc::{
    anyhow::{self, anyhow, bail, Context},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, Default)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rope<const LEN: usize> {
    knots: [Knot; LEN],
}

impl<const LEN: usize> Default for Rope<LEN> {
    fn default() -> Self {
        Self {
            knots: [Knot::default(); LEN],
        }
    }
}

impl<const LEN: usize> Rope<LEN> {
    fn move_head(&mut self, dx: i32, dy: i32) {
        let mut new_knots = self.knots;
        new_knots[0].x += dx;
        new_knots[0].y += dy;

        for tail_index in 1..LEN {
            let head_index = tail_index - 1;

            let new_head = new_knots[head_index];
            let old_head = self.knots[head_index];
            let mut tail = new_knots[tail_index];

            if (new_head.x - tail.x).abs() >= 2 || (new_head.y - tail.y).abs() >= 2 {
                tail = old_head;
            }

            new_knots[tail_index] = tail;
        }

        self.knots = new_knots;
    }

    fn head(&self) -> &Knot {
        &self.knots[0]
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }
}

#[derive(Default)]
struct History<const LEN: usize> {
    entries: Vec<Rope<LEN>>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl<const LEN: usize> History<LEN> {
    fn move_head(&mut self, rope: &mut Rope<LEN>, dx: i32, dy: i32) {
        rope.move_head(dx, dy);
        self.entries.push(*rope);
        (self.min_x, self.min_y) = (self.min_x.min(rope.head().x), self.min_y.min(rope.head().y));
        (self.max_x, self.max_y) = (self.max_x.max(rope.head().x), self.max_y.max(rope.head().y));
    }
}

fn count_visited_tiles<const LEN: usize>(challenge: &Challenge) -> anyhow::Result<usize> {
    let mut rope = Rope::<LEN>::default();
    let mut history = History::<LEN>::default();
    history.entries.push(rope);

    for line in challenge.input.lines() {
        let Some((direction, step_count)) = line.split_once(' ')
        else { bail!("line is not formatted properly: {line}") };
        let step_count = step_count.parse::<usize>().context("invalid step count")?;
        let (dx, dy) = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => bail!("invalid direction: {line}"),
        };
        for _ in 0..step_count {
            history.move_head(&mut rope, dx, dy);
        }
    }

    if challenge.debug_flags.contains("history") {
        for entry in &history.entries {
            println!("{entry:?}");
            for y in history.min_y..=history.max_y {
                for x in history.min_x..=history.max_x {
                    print!(
                        "{}",
                        if x == entry.head().x && y == entry.head().y {
                            'H'
                        } else if entry
                            .knots
                            .iter()
                            .skip(1)
                            .any(|knot| knot.x == x && knot.y == y)
                        {
                            'T'
                        } else {
                            '.'
                        }
                    )
                }
                println!();
            }
            println!();
        }
        println!("---");
    }

    let visited_tiles = history
        .entries
        .iter()
        .map(|rope| (rope.tail().x, rope.tail().y))
        .collect::<HashSet<_>>();
    if challenge.debug_flags.contains("tail") {
        for y in history.min_y..=history.max_y {
            for x in history.min_x..=history.max_x {
                print!(
                    "{}",
                    if visited_tiles.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
            println!();
        }
    }

    Ok(visited_tiles.len())
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let part_1 = count_visited_tiles::<2>(&challenge).context("part 1 failed")?;
    println!("part 1: {part_1}");

    let part_2 = count_visited_tiles::<10>(&challenge).context("part 2 failed")?;
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
