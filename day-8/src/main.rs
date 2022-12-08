use std::{convert::Infallible, ops::Index, str::FromStr};

use aoc::{
    anyhow::{self, anyhow, bail, Context},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tree {
    height: u8,
}

impl Tree {
    fn exposes(&self, other: &Tree) -> bool {
        self.height < other.height
    }
}

#[derive(Debug, Clone)]
struct Forest {
    width: u32,
    depth: u32,
    trees: Vec<Tree>,
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: Option<u32> = None;
        let mut depth = 0;
        let mut trees = vec![];
        for line in s.lines() {
            if let Some(width) = width {
                if line.len() as u32 != width {
                    bail!(
                        "all lines must be the same width (first line's width was {width}): {line}"
                    );
                }
            }
            trees.extend(line.chars().map(|c| Tree {
                height: (c as u32) as u8 - b'0',
            }));
            width = Some(line.len() as u32);
            depth += 1;
        }
        Ok(Forest {
            width: width.unwrap_or(0),
            depth,
            trees,
        })
    }
}

impl Forest {
    fn flatten_index(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.width as i32) as usize
    }

    fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.depth as i32
    }
}

impl Index<(i32, i32)> for Forest {
    type Output = Tree;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        if self.is_in_bounds(index) {
            &self.trees[self.flatten_index(index)]
        } else {
            &Tree { height: 0 }
        }
    }
}

impl Forest {
    fn is_visible(&self, (x, y): (i32, i32)) -> bool {
        if x == 0 || y == 0 || x == self.width as i32 - 1 || y == self.depth as i32 - 1 {
            return true;
        }

        let center = self[(x, y)];

        let left_visible = (-1..x).all(|xx| self[(xx, y)].exposes(&center));
        let right_visible = (x + 1..=self.width as i32).all(|xx| self[(xx, y)].exposes(&center));
        let top_visible = (-1..y).all(|yy| self[(x, yy)].exposes(&center));
        let bottom_visible = (y + 1..=self.depth as i32).all(|yy| self[(x, yy)].exposes(&center));

        left_visible || right_visible || top_visible || bottom_visible
    }

    /// Shoot a ray from (x, y) in the direction (dx, dy). Returns the number of steps taken before
    /// an obstruction is encountered.
    fn raycast(&self, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) -> usize {
        if !self.is_in_bounds((x + dx, y + dy)) {
            return 0;
        }

        let center = self[(x, y)];
        let mut steps = 0;
        loop {
            x += dx;
            y += dy;
            if !self.is_in_bounds((x, y)) {
                break;
            }
            steps += 1;
            if !self[(x, y)].exposes(&center) {
                break;
            }
        }
        steps
    }

    fn scenic_score(&self, position: (i32, i32)) -> usize {
        let left_view_distance = self.raycast(position, (-1, 0));
        let right_view_distance = self.raycast(position, (1, 0));
        let top_view_distance = self.raycast(position, (0, -1));
        let bottom_view_distance = self.raycast(position, (0, 1));
        left_view_distance * right_view_distance * top_view_distance * bottom_view_distance
    }

    fn positions(&self) -> impl Iterator<Item = (u32, u32)> {
        let (width, depth) = (self.width, self.depth);
        (0..depth).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let forest = challenge
        .input
        .parse::<Forest>()
        .context("cannot parse forest")?;

    if challenge.debug_output {
        for y in 0..forest.depth as i32 {
            for x in 0..forest.width as i32 {
                print!("{}", if forest.is_visible((x, y)) { '#' } else { ' ' });
            }
            println!();
        }
    }

    let visible_count = forest
        .positions()
        .filter(|&(x, y)| forest.is_visible((x as i32, y as i32)))
        .count();
    println!("part 1: {visible_count}");

    if challenge.debug_output {
        println!();
        for y in 0..forest.depth as i32 {
            for x in 0..forest.width as i32 {
                print!("{:4} ", forest.scenic_score((x, y)));
            }
            println!();
        }
    }

    let max_scenic_score = forest
        .positions()
        .map(|(x, y)| forest.scenic_score((x as i32, y as i32)))
        .max()
        .ok_or_else(|| anyhow!("there are no trees to iterate over"))?;
    println!("part 2: {max_scenic_score}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
