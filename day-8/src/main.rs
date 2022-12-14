use std::str::FromStr;

use aoc::{
    anyhow::{self, anyhow, Context},
    bitmap::{Bitmap, BitmapParser},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Tree {
    height: u8,
}

impl Tree {
    fn exposes(&self, other: &Tree) -> bool {
        self.height < other.height
    }
}

struct TreeParser;

impl BitmapParser for TreeParser {
    type Element = Tree;

    fn parse_element(&mut self, _: (u32, u32), c: char) -> Option<Self::Element> {
        Some(Tree {
            height: (c as u32) as u8 - b'0',
        })
    }
}

struct Forest {
    bitmap: Bitmap<Tree>,
}

impl Forest {
    fn is_visible(&self, (x, y): (i32, i32)) -> bool {
        if x == 0
            || y == 0
            || x == self.bitmap.width as i32 - 1
            || y == self.bitmap.height as i32 - 1
        {
            return true;
        }

        let center = self.bitmap[(x, y)];

        let left_visible = (-1..x).all(|xx| self.bitmap[(xx, y)].exposes(&center));
        let right_visible =
            (x + 1..=self.bitmap.width as i32).all(|xx| self.bitmap[(xx, y)].exposes(&center));
        let top_visible = (-1..y).all(|yy| self.bitmap[(x, yy)].exposes(&center));
        let bottom_visible =
            (y + 1..=self.bitmap.height as i32).all(|yy| self.bitmap[(x, yy)].exposes(&center));

        left_visible || right_visible || top_visible || bottom_visible
    }

    /// Shoot a ray from (x, y) in the direction (dx, dy). Returns the number of steps taken before
    /// an obstruction is encountered.
    fn raycast(&self, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) -> usize {
        if !self.bitmap.is_in_bounds((x + dx, y + dy)) {
            return 0;
        }

        let center = self.bitmap[(x, y)];
        let mut steps = 0;
        loop {
            x += dx;
            y += dy;
            if !self.bitmap.is_in_bounds((x, y)) {
                break;
            }
            steps += 1;
            if !self.bitmap[(x, y)].exposes(&center) {
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
        let (width, depth) = (self.bitmap.width, self.bitmap.height);
        (0..depth).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bitmap: Bitmap::parse(TreeParser, s)?.0,
        })
    }
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let forest = challenge
        .input
        .parse::<Forest>()
        .context("cannot parse forest")?;

    if challenge.debug_flags.contains("visibility") {
        for y in 0..forest.bitmap.height as i32 {
            for x in 0..forest.bitmap.width as i32 {
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

    if challenge.debug_flags.contains("scenic-score") {
        println!();
        for y in 0..forest.bitmap.height as i32 {
            for x in 0..forest.bitmap.width as i32 {
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
